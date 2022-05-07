use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::{Duration};

use crate::sensors_data_generation::{humidity_data_generation, temperature_data_generation};
use crate::data_types::data_types::{JsonDataMessage, Telegram, ProgramConfig, JsonErrorMessage};
use crate::inner_logic::inner_logic::{copy_mutex, create_mutex};

mod sensors_data_generation;
mod data_types;
mod networking;
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

    let mut config = ProgramConfig{
        url: "http://localhost:1880/posting_link".to_string(),
        sensors_errors: 0,
        sending_errors: 0
    };

    let mut sensor_signals = (false, false);

    let sensor_mutexs = create_mutex(&data);
    //ссылки мьютексов для спокойной передачи владения без последствий
    let sensors_mutex_copy = copy_mutex(&sensor_mutexs);


    let (temperature_tx, temperature_rx) = mpsc::channel();
    let (humidity_tx, humidity_rx) = mpsc::channel();

    let temperature_thread = thread::spawn(move || loop {
        temperature_data_generation(&sensors_mutex_copy.0, &temperature_tx);
    });
    let humidity_thread = thread::spawn(move || loop {
        humidity_data_generation(&sensors_mutex_copy.1, &humidity_tx);
    });


    // цикл чтения сигналов по дедлайну.
    // дедлайн прошел - ошибка и отправка сообщения об ошибке.
    //примечание: да, тут уже синхронные функции. nuff said
    loop{

        match temperature_rx.recv_timeout(Duration::from_secs(5)) {
            Ok(_) => {
                sensor_signals.0 = true;
            }
            Err(_) => {

            }
        }

        match humidity_rx.recv_timeout(Duration::from_secs(5)) {
            Ok(_) => {
                sensor_signals.1 = true;
            }
            Err(_) => {

            }
        }

        if sensor_signals.0 || sensor_signals.1{
            let temp_data_obj = JsonDataMessage::init_via_mutex(sensor_mutexs.0.lock().unwrap(),
                                                                sensor_mutexs.1.lock().unwrap()).serialization();

        } else {
            config.sensors_errors +=1;
            if config.sensors_errors == 3{
                break;
            }
        }

    }
}
