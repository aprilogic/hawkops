use clap::{Arg, Command};
use dotenv::dotenv;
use std::env;

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

// fn fetch_jwt() -> Result<String, reqwest::Error> {
//     let client = reqwest::Client::new();
//     let res = client
//         .get("https://api.stackhawk.com/api/v1/auth/login")
//         .header("Content-Type", "application/json")
//         .header("Accept", "application/json")
//         .header("X-ApiKey", env::var("HAWK_API_KEY").unwrap())
//         .send()
//         .text();
// }