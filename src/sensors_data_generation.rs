
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use rand::Rng;
use std::thread::sleep;
use std::time::{Duration};

//TODO: сделать на условных переменных может быть?
pub fn temperature_data_generation(local_mutex: &Arc<Mutex<(i32, i32)>>, sender: &Sender<bool>){
    let mut local_int_tuple = local_mutex.lock().unwrap();
    let mut rng = rand::thread_rng();

    //TODO: сделать АШИБОЧКУ показаний для нормального теста
    /*for current_temperature_sensor in local_int_tuple {
        if current_temperature_sensor == 0 {
            current_temperature_sensor = rng.gen_range(15..24);
        } else {
            current_temperature_sensor += rng.gen_range(-2..2);
            if current_temperature_sensor > 23 {
                current_temperature_sensor -= 1;
            }
        }
    }*/

    drop(local_int_tuple);

    sender.send(true).unwrap();

    if rng.gen_range(0..10) < 8 {
        sleep(Duration::from_secs(3));
    } else {
        sleep(Duration::from_secs(6));
    }

}

pub fn humidity_data_generation(local_mutex: &Arc<Mutex<(i32, i32)>>, sender: &Sender<bool>){
    let mut local_int_tuple = local_mutex.lock().unwrap();
    let mut rng = rand::thread_rng();
    /*
    //TODO: сделать АШИБОЧКУ показаний (типа выход за предел 65) для нормального теста
    for current_humidity_sensor in local_int_tuple {
        if current_humidity_sensor == 0 {
            current_humidity_sensor = rng.gen_range(35..55);
        } else {
            current_humidity_sensor += rng.gen_range(-3..4);
            if current_humidity_sensor > 60 {
                current_humidity_sensor -= 1;
            }
        }
    }
    */
    drop(local_int_tuple);

    sender.send(true).unwrap();

    if rng.gen_range(0..10) < 8 {
        sleep(Duration::from_secs(3));
    } else {
        sleep(Duration::from_secs(6));
    }

}