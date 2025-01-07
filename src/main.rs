use clap::{Arg, Command};
use dotenv::dotenv;
use std::env;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json;
use config::Config;

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

// `hawkops auth login` command
fn ops_auth_login() -> Result<(), String> {
    let jwt = fetch_jwt()?;
    println!("Logged in successfully. Retrieved JWT:\n{}", jwt);
    check_jwt_expiration(&jwt);
    Ok(())
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