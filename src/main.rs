use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::RecvTimeoutError;
use std::thread;
use std::time::{Duration};

use crate::data_checking::check_data;
use crate::sensors_data_generation::{humidity_data_generation, temperature_data_generation};
use crate::data_types::data_types::{JsonMessage, Telegram};
use crate::inner_logic::inner_logic::{copy_mutex, create_mutex};

mod sensors_data_generation;
mod data_types;
mod networking;
mod data_checking;
mod inner_logic;

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

    let sensor_mutexs = create_mutex(&data);
    //ссылки мьютексов для спокойной передачи владения
    let sensors_mutex_copy = copy_mutex(&sensor_mutexs);

    let (temperature_tx, temperature_rx) = mpsc::channel();
    let (humidity_tx, humidity_rx) = mpsc::channel();
    //let (checker_tx, checker_rx) = mpsc::channel();

    let temperature_thread = thread::spawn(move || loop {
        temperature_data_generation( &sensors_mutex_copy.0, &temperature_tx);
    });
    let humidity_thread = thread::spawn(move || loop {
        humidity_data_generation(&sensors_mutex_copy.1, &humidity_tx);
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
      let temp_data_obj = JsonMessage::init_via_mutex(sensor_mutexs.0.lock().unwrap(),
                                                      sensor_mutexs.1.lock().unwrap(),
                                                            "OK".to_string());
        let temp_json_obj = temp_data_obj.serialization();
        println!("{:?}", temp_json_obj);

    }
}
