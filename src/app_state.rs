use std::sync::Mutex;

pub struct AppStateWithCounter {
    pub auth_counter: Mutex<u32>,
    pub verify_counter: Mutex<u32>,
    pub auth_time: Mutex<u128>,
    pub verify_time: Mutex<u128>,
}
