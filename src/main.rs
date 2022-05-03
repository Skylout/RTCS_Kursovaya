use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use crate::data_types::data_types::Telegram;

mod sensors_data_generation;
mod data_types;
mod networking;
mod data_checking;

//Да согреет Мать Бодхо твои следы!
//Когдая писал этот код, то только я и Бос Турох знали, как
//он работает. Теперь знает только Бос Турох

fn main() {
    let mut thread_handles = vec![];
    let mut mutex_vector = vec![];
    let mut mutex_copies = vec![];
    let mut channels_vector = vec![];

    let data = Telegram{
        temperature_values: (0, 0),
        humidity_values: (0, 0),
        troubles_counter: 0
    };

    let temperature_mutex = Arc::new(Mutex::new(data.temperature_values));
    mutex_vector.push(temperature_mutex);
    let humidity_mutex = Arc::new(Mutex::new(data.humidity_values));
    mutex_vector.push(humidity_mutex);



    loop{

    }
}
