use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::RecvTimeoutError;
use std::thread;
use std::time::{Duration};

use crate::data_checking::check_data;
use crate::sensors_data_generation::{humidity_data_generation, temperature_data_generation};
use crate::data_types::data_types::Telegram;

mod sensors_data_generation;
mod data_types;
mod networking;
mod data_checking;

//Да согреет Мать Бодхо твои следы!
//Когдая писал этот код, то только я и Бос Турох знали, как
//он работает. Теперь знает только Бос Турох

fn main() {
    let mut data = Telegram{
        temperature_values: (0, 0),
        humidity_values: (0, 0),
        troubles_counter: 0
    };
    let mut sensor_signals = (false, false);

    let temperature_mutex = Arc::new(Mutex::new(data.temperature_values));
    let humidity_mutex = Arc::new(Mutex::new(data.humidity_values));

    //ссылки мьютексов для спокойной передачи владения
    let temperature_mutex_copy = Arc::clone(&temperature_mutex);
    let humidity_mutex_copy = Arc::clone(&humidity_mutex);

    let (temperature_tx, temperature_rx) = mpsc::channel();
    let (humidity_tx, humidity_rx) = mpsc::channel();
    //let (checker_tx, checker_rx) = mpsc::channel();

    let temperature_thread = thread::spawn(move || loop {
        temperature_data_generation( &temperature_mutex_copy, &temperature_tx);
    });
    let humidity_thread = thread::spawn(move || loop {
        humidity_data_generation(&humidity_mutex_copy, &humidity_tx);
    });

    // цикл чтения сигналов по дедлайну.
    // дедлайн прошел - ошибка и отправка сообщения об ошибке.
    // поток проверка и результат проверки
    // данные неправильные - ошибка и отправка сообщения об ошибке.
    loop{
        match temperature_rx.recv_timeout(Duration::from_secs(4)) {
            Ok(resp) => {
                sensor_signals.0 = resp;
            },
            Err(RecvTimeoutError::Timeout) => sensor_signals.0 = false,
            Err(RecvTimeoutError::Disconnected) => {
                // handle disconnect
            }
        };

        match humidity_rx.recv_timeout(Duration::from_secs(4)) {
            Ok(resp) => {
                sensor_signals.1 = resp;
            },
            Err(RecvTimeoutError::Timeout) => sensor_signals.1 = false,
            Err(RecvTimeoutError::Disconnected) => {
                // handle disconnect
            }
        };
        //TODO: подумать над разными типами ошибок
        if sensor_signals.0 || sensor_signals.1{
            //TODO: вытащить в отдельный поток
            //check_data(&data,&checker_tx);
        } else {

        }
        //проверенный модуль отправки
/*
        let json_local_mutex = temp_int_mutex.lock().unwrap();
        let json_local_mutex1 = temp_second_int_mutex.lock().unwrap();

        let temp_data_obj = JsonMessage{
            temperature_value1: json_local_mutex.0,
            temperature_value2: json_local_mutex.1,
            humidity_value1: json_local_mutex1.0,
            humidity_value2: json_local_mutex1.1,
            status: "OK".to_string()
        };

        let temp_json_obj = serde_json::to_string(&temp_data_obj).unwrap();
        println!("Here: {}", temp_json_obj);
        let send_thread = thread::spawn(|| {
            let send_result = send_data_via_http(temp_json_obj);

            match send_result {
                Ok(_) => {
                    println!("WE DONE");
                }
                Err(err) => {
                    println!("ERROR: {}", err);
                }
            }
        });*/
    }
}
