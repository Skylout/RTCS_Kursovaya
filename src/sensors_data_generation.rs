use rand::Rng;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn temperature_data_generation(local_mutex: &Arc<Mutex<(i32, i32)>>, sender: &Sender<bool>) {
    let mut local_int_tuple = local_mutex.lock().unwrap();
    let mut rng = rand::thread_rng();

    if local_int_tuple.0 == 0 {
        local_int_tuple.0 = rng.gen_range(20..24);
    } else {
        if local_int_tuple.0 > 25 {
            local_int_tuple.0 -= rng.gen_range(0..2);
        } else {
            local_int_tuple.0 += rng.gen_range(0..2);
        }
    }

    if rng.gen_range(0..=1000) == 1000 {
        local_int_tuple.0 = rng.gen_range(0..100);
    }

    if local_int_tuple.1 == 0 {
        local_int_tuple.1 = local_int_tuple.0;
    } else {
        if local_int_tuple.1 > 25 {
            local_int_tuple.1 -= rng.gen_range(0..2);
        } else {
            local_int_tuple.1 += rng.gen_range(0..2);
        }
    }
    if rng.gen_range(0..=1000) == 1000 {
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
        local_int_tuple.0 = rng.gen_range(50..=59);
    } else {
        if local_int_tuple.0 > 60 {
            local_int_tuple.0 -= rng.gen_range(0..2);
        } else {
            local_int_tuple.0 += rng.gen_range(0..2);
        }
    }

    if rng.gen_range(0..=1000) == 1000 {
        local_int_tuple.0 = rng.gen_range(60..100);
    }

    if local_int_tuple.1 == 0 {
        local_int_tuple.1 = local_int_tuple.0;
    } else {
        if local_int_tuple.1 > 60 {
            local_int_tuple.1 -= rng.gen_range(0..2);
        } else {
            local_int_tuple.1 += rng.gen_range(0..2);
        }
    }

    if rng.gen_range(0..=1000) == 1000 {
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
