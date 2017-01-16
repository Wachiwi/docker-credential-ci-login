extern crate base64;
extern crate regex;

use std::result::*;
use std::process::Command;

use regex::Regex;

#[derive(Debug)]
pub enum AWSError {
    ParseError,
    WeirdAWSResponse
}

pub type AWSResult = Result<(String, String), AWSError>;

fn get_aws_cli_output() -> String {
    let cmd = Command::new("aws")
            .args(&["ecr", "get-authorization-token",
                    "--query", "authorizationData[].authorizationToken",
                    "--output", "text"])
            .output()
            .expect("failed to execute process");
    return String::from_utf8_lossy(&cmd.stdout).into_owned();
}

fn decode_aws_output(out: String) -> Result<String, AWSError> {
    match base64::decode_ws(&out) {
        Ok(parsed) => {
            return Ok(String::from_utf8_lossy(&parsed).into_owned());
        },
        Err(why) => panic!("{:?}", AWSError::ParseError)
    }
}

fn parse_aws_output(out: &String) -> AWSResult {
    let decoded_ws : String = out.replace(':', " ");
    let components : Vec<&str> = decoded_ws.split_whitespace().collect();
    if components.len() == 2 {
        return Ok((String::from_utf8_lossy(components.get(0).unwrap().as_bytes()).into_owned(),
                   String::from_utf8_lossy(components.get(1).unwrap().as_bytes()).into_owned()));
    } else {
        for c in components {
            println!("{:?}", c);
        }
        return Err(AWSError::WeirdAWSResponse);
    }
}


pub fn get_authorization_information() -> AWSResult {
    let out = get_aws_cli_output();
    let decoded = decode_aws_output(out).unwrap();
    return parse_aws_output(&decoded);
}

pub fn is_aws_ecr_url(s: &String) -> bool {
    let aws_ecr_pattern : Regex = Regex::new(r"(^[a-zA-Z0-9][a-zA-Z0-9-_]*)\.dkr\.ecr\.([a-zA-Z0-9][a-zA-Z0-9-_]*)\.amazonaws\.com(\.cn)?").unwrap();
    return aws_ecr_pattern.is_match(s);
}
