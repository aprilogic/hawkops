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
                .subcommand(Command::new("login").about("Log in with your API key")),
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
            _ => println!("Use `hawkops auth login` to log in."),
        },
        _ => println!("Use --help to see available commands."),
    }
}
