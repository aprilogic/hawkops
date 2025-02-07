mod config;

use clap::{Arg, Command};
// use dotenv::dotenv;
// use std::env;
// use ::config::Config;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json;
use crate::config::{load_config, HawkOpsConfig};

fn main() {
    // Load environment variables from .env
    // dotenv().ok();
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
                        ops_auth_login(api_key).expect("Call to ops_auth_login failed")
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

// `hawkops auth login` command
fn ops_auth_login(api_key: String) -> Result<(), String> {
    println!("Logging in with API key: {}", api_key);
    let jwt = fetch_jwt(api_key)?;
    check_jwt_expiration(&jwt);
    println!("Logged in successfully. Retrieved JWT:\n{}", jwt);
    Ok(())
}

fn fetch_jwt(api_key: String) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get("https://api.stackhawk.com/api/v1/auth/login")
        .header("Accept", "application/json")
        .header("X-ApiKey", api_key)
        .send()
        .map_err(|e| e.to_string())?;
    let json: serde_json::Value = res.json().map_err(|e| e.to_string())?;
    json["token"].as_str().map(|s| s.to_string()).ok_or("Token not found".to_string())
}

fn check_jwt_expiration(jwt: &str) {
    let jwt_parts: Vec<&str> = jwt.split('.').collect();
    let claims = jwt_parts.get(1).unwrap();
    let claims = format!("{}{}", claims, "=".repeat((4 - claims.len() % 4) % 4));
    let decoded_claims = URL_SAFE.decode(claims).expect("Failed to decode claims");
    let claims_str = String::from_utf8(decoded_claims).expect("Failed to convert claims to string");
    let claims_json: serde_json::Value = serde_json::from_str(&claims_str).unwrap();
    let exp = claims_json["exp"].as_i64().unwrap();
    let now = chrono::Utc::now().timestamp();
    let time_left = exp - now;
    println!("JWT expires in {} seconds", time_left);
}

fn _ops_init() -> Result<(), String> {
    // TODO:
    // 1. Check for existing config file and prompt user to overwrite
    // 2. Prompt user for API key
    // 3. Write API key to config file
    // 4. Fetch JWT
    // 5. Write JWT to config file
    // 6. Check JWT expiration
    // 7. Print success message - "Initialized hawkops successfully"
    Ok(())
}
