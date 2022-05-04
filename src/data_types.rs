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
    }

    //снек-кейс тут не применяется по простой причине - это инородный объект,
    //его стиль подогнан под JS
    #[derive(Serialize, Deserialize)]
    pub struct JsonMessage {
        pub temperatureValue1: i32,
        pub temperatureValue2: i32,
        pub humidityValue1: i32,
        pub humidityValue2: i32,
        pub dateTime: String,
        pub status: String
    }

    impl JsonMessage {
        pub fn serialization (&self)->String{
            let temp_json_obj = serde_json::to_string(&self).unwrap();
            return  temp_json_obj;
        }

        pub fn init_via_mutex (temperature_mutex: MutexGuard<(i32,i32)>,
                           humidity_mutex: MutexGuard<(i32,i32)>,
                           status: String)->JsonMessage{
            return JsonMessage{
                temperatureValue1: temperature_mutex.0,
                temperatureValue2: temperature_mutex.1,
                humidityValue1: humidity_mutex.0,
                humidityValue2: humidity_mutex.1,
                dateTime: Local::now().to_string(),
                status: status.to_string()
            }
        }
    }
}