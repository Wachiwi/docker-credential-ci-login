extern crate base64;

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
    pub expires_at: SystemTime
}

pub struct Cache {
    pub file_name: String,
    pub file: File,
    pub cache_type: CacheType,
    pub created_at: Instant,
    entries: Vec<(String, String)>
}

#[derive(Debug)]
pub enum CacheError {
    NoEntry,
    InternalError,
    WriteCacheError
}

#[derive(Debug)]
pub enum CacheType {
    TimeBased,
    Persistent
}

pub fn init(cache_type: CacheType) -> Result<Cache, CacheError> {
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
            match create_dir_all(&dir) {
                Err(_) => return Err(CacheError::InternalError),
                _ => {}
            }
            dir.push_str("/cache");
        }
    }

    println!("{:?}", &dir);

    let mut cache = OpenOptions::new()
            .mode(0o640)
            .read(true)
            .write(true)
            .create(true)
            .open(&dir).unwrap();

    return Ok(Cache {
        file_name: dir,
        file: cache,
        cache_type: cache_type,
        created_at: Instant::now(),
        entries: load_cache(&mut cache)
    });
}

fn load_cache(cache: &mut File) -> Vec<(String, String)> {
    let mut buffer = Vec::new();
    let mut lines : Vec<(String, String)> = Vec::new();
    let length = cache.read_to_end(&mut buffer);
    let f_str = String::from_utf8_lossy(&buffer);
    for line in f_str.lines() {
        let split_line : String = line.replace(':', " ");
        let components : Vec<&str> = split_line.split_whitespace().collect();
        lines.push((String::from_utf8_lossy(components.get(0).unwrap().as_bytes()).into_owned(),
                    String::from_utf8_lossy(components.get(1).unwrap().as_bytes()).into_owned()));
        println!("L {}", line);
    }
    return lines;
}

impl Cache {
    pub fn contains_server(&mut self, server: &String) -> bool {
        // let buffer = Vec::new();
        // let length = self.file.read_to_end(&mut buffer);
        self.load_file();
        return false;
    }

    pub fn contains_entry(&self, entry: CacheEntry) -> bool {
        unimplemented!()
    }

    pub fn add_entry(&mut self, entry: CacheEntry) -> Result<bool, CacheError> {
        // if self.contains_server(&entry.server) {
        //
        // }
        // Check if entry already exists

        // Update if it exists
        // Create if not exists
        // Encode base64
        let bytes = self.encode_entry(&entry).into_bytes();
        match self.file.write_all(&bytes) {
            Ok(_) => return Ok(true),
            Err(_) => return Err(CacheError::WriteCacheError)
        }
    }

    pub fn remove_entry(&mut self, server: String) -> Result<bool, CacheError> {
        unimplemented!()
    }

    pub fn get_entry(&self, server: String) -> Result<CacheEntry, CacheError> {
        unimplemented!()
    }

    pub fn get_entries(&self) -> Result<Vec<CacheEntry>, CacheError> {
        //let entries = self.load_file();

        unimplemented!()
    }

    fn load_file(&mut self) -> Vec<(String, String)>{
        let mut buffer = Vec::new();
        let mut lines : Vec<(String, String)> = Vec::new();
        let length = self.file.read_to_end(&mut buffer);
        let f_str = String::from_utf8_lossy(&buffer);
        for line in f_str.lines() {
            let split_line : String = line.replace(':', " ");
            let components : Vec<&str> = split_line.split_whitespace().collect();
            lines.push((String::from_utf8_lossy(components.get(0).unwrap().as_bytes()).into_owned(),
                        String::from_utf8_lossy(components.get(1).unwrap().as_bytes()).into_owned()));
            println!("L {}", line);
        }
        return lines;
    }

    fn encode_entry(&self, entry: &CacheEntry) -> String {
        let encoded_values = base64::encode(&format!("{}:{}", &entry.username, &entry.token).into_bytes());
        return format!("{}:{}", &entry.server, encoded_values);
    }
}
