use chrono::Local;
use std::env;
use std::env::args;
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::data_types::data_types::{JsonDataMessage, JsonErrorMessage, ProgramConfig, Telegram};
use crate::inner_logic::inner_logic::{copy_mutex, create_mutex};
use crate::networking::send_data_via_http;
use crate::sensors_data_generation::{humidity_data_generation, temperature_data_generation};

mod data_types;
mod inner_logic;
mod networking;
mod sensors_data_generation;

//Да согреет Мать Бодхо твои следы!
//Когда я писал этот код, то только я и Бос Турох знали, как
//он работает. Теперь знает только Бос Турох

fn main() {
    //а вот раст может только так в аргументы команднйо строки
    let args: Vec<String> = env::args().collect();
    let mut timeout_rcv: u64= 5;
    let mut timeout_before_http: u64 = 0;

    if args.len() == 1 {
        println!("Lack of URL. Please, write correct URL.\n");
        exit(0);
    } else {
        if args.get(2) != None{
            timeout_before_http = args.get(2).unwrap().to_string().parse::<u64>().unwrap();
        }
        if args.get(3) != None{
            timeout_rcv = args.get(3).unwrap().to_string().parse::<u64>().unwrap();
        }
    };

    let data = Telegram {
        temperature_values: (0, 0),
        humidity_values: (0, 0),
        troubles_counter: 0,
    };


    let mut config = ProgramConfig {
        url: args[1].clone().to_string(),
        prefix: "/hook".to_string(),
        sensor_data_endpoint: "/process-sensor-data".to_string(),
        owner_data_endpoint: "/process-change-owner".to_string(),
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
    //примечание: да, тут уже синхронные функции, что некруто. Думать как вынести
    //отправку в отедльный поток, конечно, хорошо, но времени нет. самая простая идея -
    //привязать условную переменную
    loop {
        //let mut server_response: String;

        match temperature_rx.recv_timeout(Duration::from_secs(timeout_rcv.clone())) {
            Ok(_) => {
                sensor_signals.0 = true;
            }
            Err(_) => {
                let error_message = JsonErrorMessage {
                    error_desc: "Temperature sensors error: reading timeout".to_string(),
                    date_time: Local::now().to_string(),
                }
                .serialize();
                send_data_via_http(error_message,
                                   &config.url,
                                   &config.prefix,
                                   &config.sensor_data_endpoint);
            }
        }

        match humidity_rx.recv_timeout(Duration::from_secs(timeout_rcv.clone())) {
            Ok(_) => {
                sensor_signals.1 = true;
            }
            Err(_) => {
                let error_message = JsonErrorMessage {
                    error_desc: "Temperature sensors error: reading timeout".to_string(),
                    date_time: Local::now().to_string(),
                }
                .serialize();
                send_data_via_http(error_message,
                                   &config.url,
                                   &config.prefix,
                                   &config.sensor_data_endpoint);
            }
        }

        if sensor_signals.0 || sensor_signals.1 {
            let temp_data_obj = JsonDataMessage::init_via_mutex(
                sensor_mutexs.0.lock().unwrap(),
                sensor_mutexs.1.lock().unwrap(),
            )
            .serialization();
            sleep(Duration::from_secs(timeout_before_http.clone()));
            send_data_via_http(temp_data_obj,
                               &config.url,
                               &config.prefix,
                               &config.sensor_data_endpoint);
        } else {
            config.sensors_errors += 1;
            if config.sensors_errors == 3 {
                break;
            }
        }
    }
}
