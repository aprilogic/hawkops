use std::path::PathBuf;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use std::fs::File;
use std::io::{Read, Write};
use dirs;
use serde_json::from_str;
use crate::config::HawkOpsConfig;

// `hawkops auth login` command
pub fn ops_auth_login(api_key: String) {
    println!("Logging in...");
    let jwt = String::from(request_jwt(api_key));
    check_jwt_expiration(&jwt);
    write_jwt(&jwt);
    println!("Logged in successfully.");
    // println!("Logged in successfully. Retrieved JWT:\n{}", jwt);
}


fn _get_api_key(config: HawkOpsConfig) -> String {
    let api_key = match config.api_key {
        Some(api_key) => api_key,
        None => {
            eprintln!("No API key found");
            std::process::exit(1)
        }
    };
    api_key
}

// Get a valid JWT from cache or authentication
fn get_valid_jwt(api_key: String) -> String {
    let jwt = match read_jwt() {
        Some(jwt) => jwt,
        None => request_jwt(api_key)
    };
    jwt
}

fn request_jwt(api_key: String) -> String {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.stackhawk.com/api/v1/auth/login")
        .header("Accept", "application/json")
        .header("X-ApiKey", api_key)
        .send()
        .expect("Failed to send request.");
        // .map_err(|e| e.to_string())?;
    let json_response: serde_json::Value = response.json()
        .expect("Failed to parse JSON response");
    let jwt = json_response["token"]
        .as_str()
        .expect("Token not found in response")
        .to_string();
    jwt
}

//     let json: serde_json::Value = res.json().map_err(|e| e.to_string())?;
//     json["token"].as_str().map(|s| s.to_string()).ok_or("Token not found".to_string());
//
//     jwt
// }

fn check_jwt_expiration(jwt: &str) {
    let jwt_parts: Vec<&str> = jwt.split('.').collect();
    let claims = jwt_parts.get(1).unwrap();
    let claims = format!("{}{}", claims, "=".repeat((4 - claims.len() % 4) % 4));
    let decoded_claims = URL_SAFE.decode(claims).expect("Failed to decode claims");
    let claims_str = String::from_utf8(decoded_claims).expect("Failed to convert claims to string");
    let claims_json: serde_json::Value = from_str(&claims_str).unwrap();
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

fn write_jwt(jwt: &str) {
    let jwt_file_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".hawkops/.token");
    let jwt_file = File::create(jwt_file_path);
    jwt_file
        .expect("file should have been present")
        .write_all(jwt.as_ref())
        .expect("file should have been writable");
    println!("Wrote the JWT to a fie.")
}

fn read_jwt() -> Option<String> {
    let jwt_file_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".hawkops/.token");
    let mut jwt_file = File::open(jwt_file_path).ok()?;
    let mut jwt_file_contents = String::new();
        jwt_file.read_to_string(&mut jwt_file_contents).ok()?;
    //TODO: check to make sure this is a valid JWT string
    let jwt = jwt_file_contents;
    Some(jwt)
}

pub fn ops_auth_whoami(api_key: String) {
    let jwt = get_valid_jwt(api_key);
    let bearer_token = format!("Bearer {jwt}");
    let api_client = reqwest::blocking::Client::new();
    let api_response = api_client
        .get("https://api.stackhawk.com/api/v1/user")
        .header("Accept", "application/json")
        .header("Authorization", bearer_token )
        .send()
        .expect("Failed to send request.");
    // println!("Whoami response?\n{api_response:?}");
    let json_response: serde_json::Value = api_response.json()
        .expect("Failed to parse JSON response");
    let full_name: String = json_response["user"]["external"]["fullName"].to_string()
        .strip_prefix("\"").expect("ouch!")
        .strip_suffix("\"").expect("oof!").to_string();
    let email: String = json_response["user"]["external"]["email"].to_string()
        .strip_prefix("\"").expect("yeesh!")
        .strip_suffix("\"").expect("argh!").to_string();
    println!("{full_name} <{email}>");
    // println!("Whoami?\n{json_response:?}")
}
