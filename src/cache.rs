use std::time::{SystemTime, Duration};

pub struct CacheEntry {
    server: String,
    username: String,
    token: String,
    expires_at: SystemTime
}

pub enum CacheError {
    NoEntry,
    InternalError,
}

pub fn init() {

}

pub fn contains_server(server: String) -> bool {

}

pub fn contains_entry(entry: CacheEntry) -> bool {

}

pub fn add_entry(entry: CacheEntry) -> Result<bool, CacheError> {

}

pub fn remove_entry(server: String) -> Result<bool, CacheError> {

}

pub fn get_entry(server: String) -> CacheEntry {

}
