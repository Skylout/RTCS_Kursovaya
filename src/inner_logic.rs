pub mod inner_logic {
    use crate::Telegram;
    use std::sync::{Arc, Mutex};

    pub fn create_mutex(
        current_data: &Telegram,
    ) -> (Arc<Mutex<(i32, i32)>>, Arc<Mutex<(i32, i32)>>) {
        let temperature_mutex = Arc::new(Mutex::new(current_data.temperature_values));
        let humidity_mutex = Arc::new(Mutex::new(current_data.humidity_values));

        return (temperature_mutex, humidity_mutex);
    }

    pub fn copy_mutex(
        original_mutexs: &(Arc<Mutex<(i32, i32)>>, Arc<Mutex<(i32, i32)>>),
    ) -> (Arc<Mutex<(i32, i32)>>, Arc<Mutex<(i32, i32)>>) {
        return (
            Arc::clone(&original_mutexs.0),
            Arc::clone(&original_mutexs.1),
        );
    }
}
