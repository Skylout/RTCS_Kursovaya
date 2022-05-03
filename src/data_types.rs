pub mod data_types {
    use chrono;
    use serde::{Deserialize, Serialize};

    pub struct Telegram {
        pub temperature_values: (i32, i32),
        pub humidity_values: (i32, i32),
        pub troubles_counter: i32
    }

    struct ProgramConfig {
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
        pub status: String
    }
}