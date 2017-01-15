
use std::env;
use std::process;
use std::slice::Iter;

#[macro_use]
extern crate log;
extern crate env_logger;

use self::Subcommand::*;

#[derive(Debug)]
pub enum Subcommand {
    Get,
    Store,
    Erase,
    Help
}

impl std::string::ToString for Subcommand {
    fn to_string(&self) -> String {
        return format!("{:?}", self).to_lowercase();
    }
}

// impl std::convert::From<String> for Subcommand {
//     fn from(s: String) -> Self {
//         let mut c = Help;
//         for cmd in Subcommands::iterator() {
//             if s == cmd.to_string() {
//                 c = *cmd.take();
//             }
//         }
//         return c;
//     }
// }

struct Subcommands {}

impl Subcommands {
    fn iterator() -> Iter<'static, Subcommand> {
        static SUBCOMMANDS: [Subcommand;  3] = [Get, Store, Erase];
        return SUBCOMMANDS.into_iter();
    }
}

fn print_usage() {
    println!("Usage:\n\tdocker-credential-ci-login <subcommand>\nSubcommands:");
    for cmd in Subcommands::iterator() {
        println!("\t* {}", cmd.to_string());
    }
}

fn verify_subcommand(cmd: &String) -> bool {
    for c in Subcommands::iterator() {
        if c.to_string() == *cmd {
            return true;
        }
    }
    return false;
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
            process::exit(64);
        }
    }

    if ! verify_subcommand(&subcommand) {
        println!("[ERROR] Invalid subcommand");
        print_usage();
        process::exit(64);
    }

    env_logger::init().unwrap();
    info!("Hello World")

}
