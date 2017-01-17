use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::env;

use std::time::{Instant, SystemTime};

use self::CacheType::*;

pub struct CacheEntry {
    pub server: String,
    pub username: String,
    pub token: String,
    pub expires_at: Instant
}

pub struct Cache {
    pub file_name: String,
    pub file: File,
    pub cache_type: CacheType,
    pub created_at: Instant
}

pub enum CacheError {
    NoEntry,
    InternalError,
}

pub enum CacheType {
    TimeBased,
    Persistent
}

pub fn init(cache_type: CacheType) -> Cache {
    let mut dir : String;

    match cache_type {
        TimeBased => {
            let mut p = env::temp_dir();
            p.push("cache");
            dir = p.to_str().unwrap().to_owned();
        },
        Persistent => {
            let mut p = env::home_dir().unwrap();
            p.push(".ci-login");
            dir = p.to_str().unwrap().to_owned();
            create_dir_all(&dir);
            dir.push_str("/cache");
        }
    }

    println!("{:?}", &dir);

    let file = OpenOptions::new()
            .mode(0o640)
            .read(true)
            .write(true)
            .create(true)
            .open(&dir).unwrap();

    return Cache {
        file_name: dir,
        file: file,
        cache_type: cache_type,
        created_at: Instant::now()
    };
}

impl Cache {
    pub fn contains_server(&self, server: &String) -> bool {
        let buffer = Vec::new();
        let length = self.file.read_to_end(&mut buffer);
        return false;
    }

    pub fn contains_entry(&self, entry: CacheEntry) -> bool {
        unimplemented!()
    }

    pub fn add_entry(&self, entry: CacheEntry) -> Result<bool, CacheError> {
        // Check if entry already exists

        // Update if it exists
        // Create if not exists
        // Encode base64
        // format!("{}:{}", username, password);
        unimplemented!()
    }

    pub fn remove_entry(&self, server: String) -> Result<bool, CacheError> {
        unimplemented!()
    }

    pub fn get_entry(&self, server: String) -> CacheEntry {
        unimplemented!()
    }

    fn save(&self) -> Result<(), CacheError> {
        Err(());
    }

    fn load_file(&self) {

    }
}
