use rand::Rng;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn temperature_data_generation(local_mutex: &Arc<Mutex<(i32, i32)>>, sender: &Sender<bool>) {
    let mut local_int_tuple = local_mutex.lock().unwrap();
    let mut rng = rand::thread_rng();

    if local_int_tuple.0 == 0 {
        local_int_tuple.0 = rng.gen_range(15..24);
    } else {
        local_int_tuple.0 += rng.gen_range(-2..2);
        if local_int_tuple.0 > 25 {
            local_int_tuple.0 -= rng.gen_range(0..3);
        }
    }

    if rng.gen_range(0..101) > 98 {
        local_int_tuple.0 = rng.gen_range(0..100);
    }

    if local_int_tuple.1 == 0 {
        local_int_tuple.1 = local_int_tuple.0;
    } else {
        local_int_tuple.1 += rng.gen_range(-2..2);
        if local_int_tuple.1 > 25 {
            local_int_tuple.1 -= rng.gen_range(0..3);
        }
    }
    if rng.gen_range(0..101) > 98 {
        local_int_tuple.1 = rng.gen_range(0..100);
    }

    drop(local_int_tuple);

    sender.send(true).unwrap();

    if rng.gen_range(0..10) < 9 {
        sleep(Duration::from_secs(3));
    } else {
        sleep(Duration::from_secs(6));
    }
}

pub fn humidity_data_generation(local_mutex: &Arc<Mutex<(i32, i32)>>, sender: &Sender<bool>) {
    let mut local_int_tuple = local_mutex.lock().unwrap();
    let mut rng = rand::thread_rng();

    if local_int_tuple.0 == 0 {
        local_int_tuple.0 = rng.gen_range(35..55);
    } else {
        local_int_tuple.0 += rng.gen_range(-2..2);
        if local_int_tuple.0 > 60 {
            local_int_tuple.0 -= rng.gen_range(0..3);
        }
    }

    if rng.gen_range(0..101) > 98 {
        local_int_tuple.0 = rng.gen_range(60..100);
    }

    if local_int_tuple.1 == 0 {
        local_int_tuple.1 = local_int_tuple.0;
    } else {
        local_int_tuple.1 += rng.gen_range(-2..2);
        if local_int_tuple.1 > 60 {
            local_int_tuple.1 -= rng.gen_range(0..3);
        }
    }

    if rng.gen_range(0..101) > 98 {
        local_int_tuple.1 = rng.gen_range(60..100);
    }

    drop(local_int_tuple);

    sender.send(true).unwrap();

    if rng.gen_range(0..10) < 9 {
        sleep(Duration::from_secs(3));
    } else {
        sleep(Duration::from_secs(6));
    }
}
