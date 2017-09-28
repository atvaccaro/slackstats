///! A library for analyzing Slack message exports.
///

extern crate serde_json;
use serde_json::{Value, Error};

use std::error;
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run(config: Config) -> Result<(), Box<error::Error>> {
    /// The actual method to run the statistical analysis.

    //read in users
    let mut userfile_path = PathBuf::from(&config.base_path);

    if userfile_path.to_str().unwrap().contains("/") {
        userfile_path.push("users.json");
    } else {
        userfile_path.push("/users.json");
    }

    let mut userfile = fs::File::open(userfile_path)?;
    let mut contents = String::new();
    userfile.read_to_string(&mut contents);

    let v: Value = serde_json::from_str(&contents[..])?;
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert(String::from("USLACKBOT"), User {
        id: String::from("USLACKBOT"),
        username: String::from("slackbot"),
    });
    for user in v.as_array().unwrap() {
        users.insert(
            String::from(user["id"].as_str().unwrap()),
            User {
                id: String::from(user["id"].as_str().unwrap()),
                username: String::from(user["name"].as_str().unwrap()),
            }
        );
    }

    //get all messages
    let entries = fs::read_dir(config.base_path).unwrap();
    let mut messages: HashMap<String, Vec<Message>> = HashMap::new();
    messages.insert(String::from("USLACKBOT"), Vec::new());

    for user in users.keys() {
        messages.insert(user.clone(), Vec::new());
    }

    let messages_ptr = Arc::new(Mutex::new(messages));
    let mut handles = vec![];

    for entry in entries {
        if entry.as_ref().unwrap().file_type().unwrap().is_file() {
            continue;
        }

        let messages_ptr = messages_ptr.clone();
        let handle = thread::spawn(move|| {
            let sub_paths = fs::read_dir(entry.unwrap().path()).unwrap();

            for sub_path in sub_paths {
                let mut f = fs::File::open(sub_path.unwrap().path()).unwrap();
                let mut contents = String::new();
                f.read_to_string(&mut contents);

                let v: Value = serde_json::from_str(&contents[..]).unwrap();

                for message in v.as_array().unwrap() {
                    if message["user"].is_string()
                        && message["text"].is_string()
                        && message["ts"].is_string() {
                        let mut messages_map = messages_ptr.lock().unwrap();
                        (*messages_map).get_mut(&String::from(message["user"].as_str().unwrap()))
                            .unwrap()
                            .push(
                                Message {
                                        user: String::from(message["user"].as_str().unwrap()),
                                        text: String::from(message["text"].as_str().unwrap()),
                                        timestamp: message["ts"].as_str().unwrap().parse().unwrap(),
                                    }
                            );
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut messages = *messages_ptr.lock().unwrap();

    //calculate total messages
    println!("Calculating total messages...");
    for (id, user) in &users {
        let count = messages.get(id).unwrap().len();
        println!("{}: {}", user.username, count);
    }

    //calculate message diversity
    println!("Calculating total unique words...");
    let mut counts: Vec<(String, usize, usize)> = Vec::new();
    for (user, user_messages) in messages {
        let mut words: Vec<String> = user_messages
            .iter()
            .map(|ref message| message.text.clone())
            .map(|ref text| text.split_whitespace().map(|s| s.to_string()).collect())
            .flat_map(|words: Vec<String>| {
                words
            }).collect();
        let orig_length = words.len();
        words.sort();
        words.dedup();
        counts.push((users.get(&user).unwrap().username.clone(), orig_length, words.len()));
    }

    counts.sort_by(|a, b| b.2.cmp(&a.2));
    for (username, orig_length, unique_len) in counts {
        println!("{}, {}, {}, {:.3}", username, orig_length, unique_len, unique_len as f64 / orig_length as f64);
    }

    Ok(())
}

pub struct Config {
    base_path: PathBuf,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let directory = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a directory."),
        };

        Ok(Config { base_path: PathBuf::from(directory) })
    }
}

pub struct Channel {

}

#[derive(Debug)]
pub struct Message {
    user: String,
    text: String,
    timestamp: f64,
}

pub struct User {
    id: String,
    username: String,
}

#[cfg(test)]
mod test
{
    #[test]
    fn test_dedup() {
        let mut words = vec![
            String::from("hello"),
            String::from("hello"),
            String::from("world")];
        words.dedup();
        assert_eq!(words.len(), 2);
    }
}

