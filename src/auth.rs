use crate::config::load_config;

fn _login() {
    let config = load_config();
    let api_key = config.unwrap().api_key.unwrap();
    let jwt = _fetch_jwt(api_key);
    println!("JWT: {:#?}", jwt);
}

fn _fetch_jwt(api_key: String) -> Option<String> {
    let url = "https://api.stackhawk.com/v1/auth/jwt";
    let client = reqwest::blocking::Client::new();
    let response = client
    .get(url)
    .header("Accept", "application/json")
    .header("X-ApiKey", api_key)
    .send();

    let body = response.unwrap().text().unwrap();
    let jwt: serde_json::Value = serde_json::from_str(&body).ok()?;
    let jwt = jwt["token"].as_str()?;
    Some(jwt.to_string())
}

