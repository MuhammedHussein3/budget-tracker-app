use gloo::storage::{LocalStorage, Storage}; 
use serde::{Deserialize, Serialize};        

pub fn save_state<T: Serialize>(key: &str, state: &T) -> Result<(), String> {
    LocalStorage::set(key, state).map_err(|e| format!("Failed to save: {:?}", e))
}

pub fn load_state<T: for<'de> Deserialize<'de>>(key: &str) -> Option<T> {
    LocalStorage::get(key).ok()
}