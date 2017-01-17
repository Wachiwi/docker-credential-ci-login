// docker-credential-ci-login
// Copyright (C) 2017  Harpoon
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#[macro_use]
extern crate log;
extern crate regex;
#[macro_use]
extern crate json;

mod subcommands;
mod aws;
mod cache;

use std::env;
use std::process;
use std::io;
use std::io::prelude::*;
use std::time::{Duration, SystemTime};

use subcommands::Subcommand;
use cache::{CacheType,CacheEntry};

fn print_usage() {
    println!("Usage:\n\tdocker-credential-ci-login <subcommand>\nSubcommands:");
    for cmd in subcommands::iterator() {
        println!("\t* {}", cmd.to_string());
    }
}

fn read_stdin() -> String {
    let stdin = io::stdin();
    let mut input : String = String::new();
    for line in stdin.lock().lines() {
        input.push_str(&line.unwrap());
    }
    return input;
}

fn get(server: String) {
    let mut cache = cache::init(CacheType::Persistent).unwrap();
    if cache.contains_server(&server) {
        if ! aws::is_aws_ecr_url(&server) {
            println!("{}", object!{
                "Username" => "",
                "Secret" => ""
            });
        } else {
            let out = aws::get_authorization_information().unwrap();
            let result = cache.add_entry(CacheEntry {
                server: server.clone(),
                username: out.0.clone(),
                token: out.1.clone(),
                expires_at: SystemTime::now() + Duration::from_secs(21600)
            });
            match result {
                Ok(status) => {
                    println!("{}", object!{
                        "Username" => out.0,
                        "Secret" => out.1
                    });
                },
                _ => {}
            }

        }


    } else {
        println!("{}", object!{
            "Username" => "token",
            "Secret" => ""
        });
    }
}

fn erase(server: String) {
    let mut cache = cache::init(CacheType::Persistent).unwrap();
    match cache.remove_entry(server) {
        _ => {}
    }
}

fn store(server: String, username: String, password: String) {
    let mut cache = cache::init(CacheType::Persistent).unwrap();
    let res = cache.add_entry(CacheEntry {
        server: server,
        username: username,
        token: password,
        // Maximum duration of cache is 1 year (even for permanent entries)
        expires_at: SystemTime::now() + Duration::from_secs(31536000)
    });

    match res {
        Err(why) => println!("{:?}", why),
        _ => {}
    }
}


fn main() {
    let mut args = env::args();
    let subcommand : String;

    match args.nth(1) {
        Some(inner) => {
            subcommand = inner;
        }
        None => {
            print_usage();
            process::exit(127);
        }
    }

    if ! subcommands::verify_subcommand(&subcommand) {
        println!("[ERROR] Invalid subcommand");
        print_usage();
        process::exit(64);
    }

    let cmd : Subcommand = Subcommand::from(subcommand);

    // Match (switch) on which command
    // ToDo: Move next three lines into logging module
    // let mut p : String = env::var("HOME").unwrap();
    // p.push_str("/.ci-login.log");
    // let mut f = File::create(p).expect("Could not create file!");

    let input = read_stdin();

    match cmd {
        Subcommand::Get => {
            get(input);
        },
        Subcommand::Erase => {
            erase(input);
        },
        Subcommand::Store => {
            match json::parse(&input) {
                Ok(parsed_json) => {
                    store(parsed_json["ServerURL"].as_str().unwrap().to_owned(),
                          parsed_json["Username"].as_str().unwrap().to_owned(),
                          parsed_json["Secret"].as_str().unwrap().to_owned());
                },
                _ => {
                    println!("[ERROR] Could not parse JSON payload!");
                    process::exit(1);
                }
            }
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }
    process::exit(0);
}
