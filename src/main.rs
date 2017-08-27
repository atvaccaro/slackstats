extern crate slackstats;

use std::env;
use std::process;
use std::io::Write;

use slackstats::Config;

fn main() {
    let mut stderr = std::io::stderr();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
       writeln!(
           &mut stderr,
           "Problem parsing arguments: {}",
           err
       ).expect("Could not write to stderr.");
        process::exit(1);
    });

    if let Err(e) = slackstats::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}