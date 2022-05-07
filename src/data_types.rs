pub mod data_types {
    use std::sync::MutexGuard;
    use chrono;
    use chrono::{DateTime, Local};
    use serde::{Deserialize, Serialize};

    pub struct Telegram {
        pub temperature_values: (i32, i32),
        pub humidity_values: (i32, i32),
        pub troubles_counter: i32
    }

    pub struct ProgramConfig {
        pub url: String,
        pub sensors_errors: i32, //погрешность
        pub sending_errors: i32
    }

    #[derive(Serialize, Deserialize)]
    pub struct JsonErrorMessage{
        pub error_desc: String,
        pub date_time: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct JsonDataMessage {
        pub temperature_value1: i32,
        pub temperature_value2: i32,
        pub humidity_value1: i32,
        pub humidity_value2: i32,
        pub date_time: String
    }

    impl JsonDataMessage {
        pub fn serialization (&self)->String{
            return  serde_json::to_string(&self).unwrap();
        }

        pub fn init_via_mutex (temperature_mutex: MutexGuard<(i32,i32)>,
                           humidity_mutex: MutexGuard<(i32,i32)>)-> JsonDataMessage {
            return JsonDataMessage {
                temperature_value1: temperature_mutex.0,
                temperature_value2: temperature_mutex.1,
                humidity_value1: humidity_mutex.0,
                humidity_value2: humidity_mutex.1,
                date_time: Local::now().to_string(),
            }
        }
    }
}