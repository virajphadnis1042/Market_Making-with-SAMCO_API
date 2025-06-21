use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Serialize)]
struct LoginRequest {
    userId: String,
    password: String,
    yob: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

pub async fn login() -> Result<String, reqwest::Error> {
    let user = env::var("SAMCO_USER").unwrap();
    let pass = env::var("SAMCO_PASS").unwrap();
    let yob = env::var("SAMCO_YOB").unwrap();

    let client = Client::new();
    let res = client
        .post("https://tradeapi.samco.in/login")
        .json(&LoginRequest {
            userId: user,
            password: pass,
            yob,
        })
        .send()
        .await?
        .json::<Value>()
        .await?;

    let token = res["token"].as_str().unwrap_or("").to_string();

    Ok(token)
}
