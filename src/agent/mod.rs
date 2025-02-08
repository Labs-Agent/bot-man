use serde_json::json;
use std::{error::Error, time::Duration};

pub async fn get_agent_response(
    agent_url: &str,
    username: &str,
    password: &str,
    query: &str,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let payload = json!({
        "text": query,
    });

    let request = client
        .post(agent_url)
        .basic_auth(username, Some(password))
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?;

    let response = request.text().await?;
    let response: serde_json::Value = serde_json::from_str(&response)?;
    let response = response[0]["text"].as_str();

    match response {
        Some(text) => Ok(text.to_string()),
        None => Err("No response text found".into()),
    }
}
