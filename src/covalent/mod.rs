use reqwest::Client;
use std::error::Error;

pub fn start_server() -> Result<(), Box<dyn Error>> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("cd ../bot-man-agent && npm start")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err("Failed to start npm server".into())
    }
}

pub fn stop_server() -> Result<(), Box<dyn Error>> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("cd ../bot-man-agent && npm stop")
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err("Failed to stop npm server".into())
    }
}

pub async fn get_available_workflows(url: String) -> Result<Vec<String>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/api/workflows", url);
    let res = client.get(url).send().await?.text().await?;
    let flows: Vec<String> = serde_json::from_str(&res)?;
    Ok(flows)
}

pub async fn create_workflow(url: String, workflow_id: u64) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let payload = serde_json::json!({
        "name": workflow_id.to_string(),
    });

    let url = format!("{}/api/workflows", url);
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?
        .text()
        .await?;

    let response: serde_json::Value = serde_json::from_str(&res)?;
    let url = response["url"].as_str().unwrap();
    Ok(url.to_string())
}

pub async fn delete_workflow(url: String, workflow_id: u64) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let payload = serde_json::json!({
        "name": workflow_id.to_string(),
    });

    let url = format!("{}/api/workflows/{}", url, workflow_id);
    let resp = client
        .delete(url)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err("Failed to delete workflow".into())
    }
}

pub async fn run_workflow(url: String, prompt: String) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let payload = serde_json::json!({
        "prompt": prompt,
    });

    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?
        .text()
        .await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_available_workflows() {
        let url = "http://localhost:3000".to_string();
        let res = get_available_workflows(url).await.unwrap();
        assert_eq!(res.len(), 0);
    }

    #[tokio::test]
    async fn test_create_workflow() {
        let url = "http://localhost:3000".to_string();
        let res = create_workflow(url, 1).await.unwrap();
        assert_eq!(res, "http://localhost:3000/api/workflows/prompt/1");
    }

    #[tokio::test]
    async fn test_delete_workflow() {
        let url = "http://localhost:3000".to_string();
        let res = delete_workflow(url, 1).await.unwrap();
        assert_eq!(res, ());
    }

    #[tokio::test]
    async fn test_run_workflow() {
        let url = create_workflow(String::from("http://localhost:3000"), 3)
            .await
            .unwrap();
        let res = run_workflow(url, "Hello".to_string()).await.unwrap();
    }
}
