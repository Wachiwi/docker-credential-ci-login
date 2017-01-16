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
extern crate env_logger;
extern crate regex;

#[macro_use]
extern crate json;

mod subcommands;
mod aws;

use std::env;
use std::process;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use subcommands::Subcommand;

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
    if aws::is_aws_ecr_url(&server) {
            // f.write_fmt(format_args!("{}\n", "Viable aws ecr url"));
            let out = aws::get_authorization_information().unwrap();
            // f.write_fmt(format_args!("{} {}\n", out.0, out.1));

            let answer = object!{
                "Username" => out.0,
                "Secret" => out.1
            };
            println!("{}", answer);

    } else {
        let answer = object!{
            "Username" => "<token>",
            "Secret" => ""
        };
        println!("{}", answer);
    }
}

fn erase(server: String) {
    // ToDo
}

fn store(server: String, username: String, password: String) {
    // ToDo
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

    env_logger::init().unwrap();

    // Init cache

    // Match (switch) on which command
    // ToDo: Move next three lines into logging module
    let mut p : String = env::var("HOME").unwrap();
    p.push_str("/.ci-login.log");
    let mut f = File::create(p).expect("Could not create file!");

    let input = read_stdin();
    f.write_fmt(format_args!("{:?} - {}\n", &cmd, input));

    match cmd {
        Subcommand::Get => {
            get(input);
        },
        Subcommand::Erase => {
            erase(input);
        },
        Subcommand::Store => {
            let parsed = json::parse(&input).unwrap();
            store(parsed["ServerURL"].as_str().unwrap().to_owned(),
                  parsed["Usernmae"].as_str().unwrap().to_owned(),
                  parsed["Secret"].as_str().unwrap().to_owned());
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }
    process::exit(0);
}
