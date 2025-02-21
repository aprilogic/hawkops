use std::path::PathBuf;
// use base64::engine::general_purpose::URL_SAFE;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
// use config::File;
use std::fs::File;
use std::io::Write;
// use config::File;
use dirs;

// `hawkops auth login` command
pub fn ops_auth_login(api_key: String) -> Result<(), String> {

    println!("Logging in with API key: {}", api_key);
    let jwt = fetch_jwt(api_key)?;
    check_jwt_expiration(&jwt);
    write_jwt(&jwt);
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


