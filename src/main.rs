use clap::{Arg, Command};
use dotenv::dotenv;
use std::env;
use base64::decode;
use serde_json;

fn main() {
    // Load environment variables from .env
    dotenv().ok();

    // Set up CLI with Clap
    let matches = Command::new("hawkops")
        .version("0.1.0")
        .author("April Conger <april@econger.com>")
        .about("CLI for StackHawk platform")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enables verbose output"),
        )
        .subcommand(
            Command::new("auth")
                .about("Authentication commands")
                .subcommand(Command::new("login").about("Log in with your API key"))
                .subcommand(Command::new("logout").about("Log out of your account"))
                .subcommand(Command::new("whoami").about("Display information about your account")),
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("auth", auth_matches)) => match auth_matches.subcommand() {
            Some(("login", _)) => {
                if let Ok(api_key) = env::var("HAWK_API_KEY") {
                    println!("Logging in with API key: {}", api_key);
                    ops_auth_login().unwrap();
                    // TODO: Add authentication logic here
                } else {
                    eprintln!("Error: HAWK_API_KEY not set in environment.");
                }
            }
            Some(("logout", _)) => {
                println!("Logging out...")
            }
            Some(("whoami", _)) => {
                println!("Displaying account information...");
            }
            _ => println!("Use `hawkops auth login` to log in."),
        },
        _ => println!("Use --help to see available commands."),
    }
}

fn fetch_jwt() -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get("https://api.stackhawk.com/api/v1/auth/login")
        .header("Accept", "application/json")
        .header("X-ApiKey", env::var("HAWK_API_KEY").unwrap())
        .send()
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = res.json().map_err(|e| e.to_string())?;
    json["token"].as_str().map(|s| s.to_string()).ok_or("Token not found".to_string())
}

fn ops_auth_login() -> Result<(), String> {
    let jwt = fetch_jwt()?;
    println!("Logged in successfully. Retrieved JWT:\n{}", jwt);
    check_jwt_expiration(&jwt);
    Ok(())
}

fn check_jwt_expiration(jwt: &str) -> Result<(), String> {
    println!("JWT:\n{}", jwt);
    let jwt_parts: Vec<&str> = jwt.split('.').collect();
    let jwt_payload = jwt_parts[1];
    println!("JWT payload:\n{}", jwt_payload);
    match decode(jwt_payload) {
        Ok(decoded_bytes) => {
            if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                println!("Decoded string: {}", decoded_str);
            } else {
                eprintln!("Error: Decoded bytes are not valid UTF-8.");
            }
        }
        Err(e) => eprintln!("Failed to decode Base64: {}", e),
    }
    let jwt_payload_decoded = // base64 decode jwt_payload
        base64::decode(jwt_payload).map_err(|e| e.to_string())?;
    println!("Decoded JWT payload:\n{}", String::from_utf8(jwt_payload_decoded.clone()).unwrap());
    let jwt_payload_json: serde_json::Value = serde_json::from_slice(&jwt_payload_decoded).unwrap();
    let jwt_exp = jwt_payload_json["exp"].as_i64().unwrap();
    let jwt_exp_date = chrono::NaiveDateTime::from_timestamp(jwt_exp, 0);
    println!("JWT expiration date: {}", jwt_exp_date);
    Ok(())
}