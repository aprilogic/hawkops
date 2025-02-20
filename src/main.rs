mod config;
mod auth;

use clap::{Arg, Command};
// use dotenv::dotenv;
// use std::env;
// use ::config::Config;

// use serde_json;
use crate::config::{load_config};
use crate::auth::ops_auth_login;

fn main() {
    let config = load_config();
    println!("{:?}", config);

    // Set up CLI with Clap
    let matches = Command::new("hawkops")
        .version("0.1.0")
        .author("April Conger <april@econger.com>")
        .about("A CLI companion to the StackHawk platform")
        .arg(
            Arg::new("verbose")
                .help("Enables verbose output")
                .short('v')
                .long("verbose")
        )
        .subcommand(Command::new("init")
            .about("Initialize API authentication")
        )
        .subcommand(
            Command::new("auth")
                .about("Authentication commands")
                .subcommand(Command::new("login").about("Log in with your API key"))
                .subcommand(Command::new("logout").about("Log out of your account"))
                .subcommand(Command::new("whoami").about("Display information about your account"))
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("auth", auth_matches)) => match auth_matches.subcommand() {
            Some(("login", _)) => {
                match config.unwrap().api_key {
                    Some(api_key) => {
                        ops_auth_login(api_key).expect("Call to ops_auth_login failed");
                        println!("Login successful");
                    }
                    None => {eprintln!("Error, API key not found")}
                }
            }
            Some(("logout", _)) => { println!("Logging out...") }
            Some(("whoami", _)) => { println!("Displaying account information..."); }
            _ => println!("Use `hawkops auth login` to log in."),
        },
        _ => println!("Use --help to see available commands."),
    }
}
