///! A library for analyzing Slack message exports.
///

extern crate serde_json;
use serde_json::{Value, Error};

use std::error;
use std::fs;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<error::Error>> {
    let entries = fs::read_dir(config.base_path).unwrap();

    for entry in entries {

        if entry.as_ref().unwrap().file_type().unwrap().is_file() {
            continue;
        }

        let sub_paths = fs::read_dir(entry.unwrap().path()).unwrap();

        for sub_path in sub_paths {
            let mut f = fs::File::open(sub_path.unwrap().path())?;
            let mut contents = String::new();
            f.read_to_string(&mut contents);

            let v: Value = serde_json::from_str(&contents[..])?;
        }
    }

    Ok(())
}

pub struct Config {
    base_path: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let directory = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a directory."),
        };

        Ok(Config { base_path: directory })
    }
}

pub struct Channel {

}

pub struct Message {

}



#[cfg(test)]
mod test
{
    #[test]
    fn it_works() {
    }
}
