pub mod data_types {
    use chrono;
    use chrono::Local;
    use serde::{Deserialize, Serialize};
    use std::sync::MutexGuard;

    pub struct Telegram {
        pub temperature_values: (i32, i32),
        pub humidity_values: (i32, i32),
        pub troubles_counter: i32,
    }

    pub struct ProgramConfig {
        pub url: String,
        pub prefix: String,
        pub sensor_data_endpoint: String,
        pub owner_data_endpoint: String,
        pub sensors_errors: i32, //погрешность
        pub sending_errors: i32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct JsonErrorMessage {
        pub error_desc: String,
        pub date_time: String,
    }

    impl JsonErrorMessage {
        pub fn serialize(&self) -> String {
            return serde_json::to_string(&self).unwrap();
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct JsonDataMessage {
        pub temperatureValue1: i32,
        pub temperatureValue2: i32,
        pub humidityValue1: i32,
        pub humidityValue2: i32,
        pub dateTime: String,
    }

    impl JsonDataMessage {
        pub fn serialization(&self) -> String {
            return serde_json::to_string(&self).unwrap();
        }

        pub fn init_via_mutex(
            temperature_mutex: MutexGuard<(i32, i32)>,
            humidity_mutex: MutexGuard<(i32, i32)>,
        ) -> JsonDataMessage {
            return JsonDataMessage {
                temperatureValue1: temperature_mutex.0,
                temperatureValue2: temperature_mutex.1,
                humidityValue1: humidity_mutex.0,
                humidityValue2: humidity_mutex.1,
                dateTime: Local::now().to_string(),
            };
        }
    }
}
