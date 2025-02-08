use serde_json::json;
use std::{error::Error, time::Duration};

pub struct Agent {
    url: String,
    username: String,
    password: String,
}

impl Agent {
    pub fn new(url: String, username: String, password: String) -> Self {
        Self {
            url,
            username,
            password,
        }
    }

    pub async fn get_response(&self, query: &str) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        let payload = json!({
            "text": query,
        });

        let request = client
            .post(&self.url)
            .basic_auth(&self.username, Some(&self.password))
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send()
            .await?;

        // println!("response: {}", request);
        let response = request.text().await?;
        let response: serde_json::Value = serde_json::from_str(&response)?;
        let response = response[0]["text"].as_str();

        match response {
            Some(text) => Ok(text.to_string()),
            None => Err("No response text found".into()),
        }
    }
}
