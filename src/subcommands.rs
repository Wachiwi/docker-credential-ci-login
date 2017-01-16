use std;
use std::slice::Iter;
use std::clone::Clone;
use self::Subcommand::*;

#[derive(Copy, Debug)]
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

impl std::convert::From<String> for Subcommand {
    fn from(s: String) -> Self {
        let mut c = Help;
        for cmd in iterator() {
            c = cmd.clone();
            if s == cmd.to_string() {
                return c;
            }
        }
        return c;
    }
}

impl Clone for Subcommand {
    fn clone(&self) -> Subcommand {
        return *self;
    }
}

pub fn iterator() -> Iter<'static, Subcommand> {
    static SUBCOMMANDS: [Subcommand;  3] = [Get, Store, Erase];
    return SUBCOMMANDS.into_iter();
}

pub fn verify_subcommand(cmd: &String) -> bool {
    for c in iterator() {
        if c.to_string() == *cmd {
            return true;
        }
    }
    return false;
}
