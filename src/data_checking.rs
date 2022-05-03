use std::sync::mpsc::Sender;
use crate::data_types::data_types::Telegram;

pub fn check_data (current_sensor_data: &Telegram, result_sender: &Sender<(bool,bool)>) {
    let mut check_result = (true,true);

    /*
    for temperature_value in current_sensor_data.temperature_values{
        if temperature_value > 30{
            check_result.0 = false;
        }
    }

    for humidity_value in current_sensor_data.temperature_values{
        if humidity_value > 60{
            check_result.1 = false;
        }
    }
    */
    result_sender.send(check_result).unwrap();
}