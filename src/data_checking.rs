
use std::sync::mpsc::{Sender,Receiver};
use crate::data_types::data_types::Telegram;

pub fn check_data (cur_rx:(Receiver<bool>,Receiver<bool>),
                   current_sensor_data: &Telegram,
                   result_sender: &Sender<(bool,bool)>) {
    let mut check_result = (true,true);


    if current_sensor_data.temperature_values.0 > 30 ||
        current_sensor_data.temperature_values.1 > 30{
        check_result.0 = false;
    }



    if current_sensor_data.humidity_values.0 > 60 ||
        current_sensor_data.humidity_values.1 > 60{
        check_result.1 = false;
    }

    result_sender.send(check_result).unwrap();
}