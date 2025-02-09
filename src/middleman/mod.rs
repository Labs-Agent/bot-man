use crate::stats::send_stats;
use serde_json::json;
use std::env;
use std::process::Command;
use std::str;

fn string_to_json(input: String) -> serde_json::Value {
    let res = match serde_json::from_str(&input) {
        Ok(json) => json,
        Err(_) => json!({
            "node": "",
            "command": "",
            "error": "Command Not Processed"
        }),
    };
    let mut res = res.as_object().unwrap().clone();
    if !res.contains_key("node") {
        res.insert("node".to_string(), json!("1"));
    }
    if !res.contains_key("command") {
        res.insert("command".to_string(), json!(""));
    }
    if !res.contains_key("error") {
        res.insert("error".to_string(), json!("Command Not Processed"));
    }
    serde_json::Value::Object(res)
}

fn state_machine(command: String, error: String) -> (String, bool, String) {
    let mut response = String::new();
    match command.as_str() {
        "/stats" => {
            response.push_str(&send_stats());
        }
        "/error" => {
            response.push_str(&format!("error: {}", error));
        }
        "/gaia start" => {
            let output = Command::new("gaianet").arg("start").output();
            match output {
                Ok(output) => {
                    let stdout = str::from_utf8(&output.stdout).expect("Failed to read stdout");
                    if let Some(url) = stdout.lines().find(|line| line.contains("https://")) {
                        if let Some(full_url) = url.split("GaiaNet").nth(1) {
                            response.push_str(full_url.trim());
                            response.push_str("\n");
                        }
                    }
                    response.push_str("gaia start success");
                }
                Err(e) => response.push_str(&format!("gaia start failed: {}", e)),
            }
        }
        "/gaia stop" => {
            let output = Command::new("gaianet").arg("stop").output();
            match output {
                Ok(_output) => response.push_str("gaia stop success"),
                Err(e) => response.push_str(&format!("gaia stop failed: {}", e)),
            }
        }
        "/cov start" => {
            response.push_str("cov start success");
        }
        "/cov stop" => {
            response.push_str("cov stop success");
        }
        command if command.starts_with("/cov flow ") => {
            let flow_number = command.trim_start_matches("/cov flow ");
            response.push_str(&format!("cov flow {} success", flow_number));
        }
        command if command.starts_with("/cov stopflow ") => {
            let flow_number = command.trim_start_matches("/cov stopflow ");
            response.push_str(&format!("cov stopflow {} success", flow_number));
        }
        "/cov info" => {
            response.push_str("cov info success");
        }
        command if command.starts_with("/run ") => {
            let command = command.trim_start_matches("/run ");
            let output = Command::new("sh").arg("-c").arg(command).output();
            match output {
                Ok(output) => {
                    let stdout = str::from_utf8(&output.stdout).expect("Failed to read stdout");
                    response.push_str(stdout);
                }
                Err(e) => response.push_str(&format!("{}", e)),
            }
        }
        command if (command).starts_with("/deploy") => {
            let github_url = command.trim_start_matches("/deploy ");
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("sh deploy.sh {}", github_url))
                .output();
            match output {
                Ok(output) => {
                    let stdout = str::from_utf8(&output.stdout).expect("Failed to read stdout");
                    let lines: Vec<&str> = stdout.lines().collect();
                    let last_line = lines.last().unwrap();
                    let res = string_to_json(last_line.to_string());
                    return (
                        format!("This is the contract address of your deployment: {}\n", res["contract address"].as_str().unwrap().to_string()),
                        true,
                        res["abi"].as_str().unwrap().to_string(),
                    );
                }
                Err(e) => response.push_str(&format!("{}", e)),
            }
        }
        _ => {
            response.push_str(&format!("{}", error));
        }
    }
    (response, false, "".to_string())
}

pub fn main_middleman(inferred_json: String) -> (String, bool, String) {
    let res = string_to_json(inferred_json);
    println!("{:?}", res);
    let node_number = env::var("NODE_NUMBER").expect("NODE_NUMBER must be set");
    let node = res["node"].as_str().unwrap();
    if node != node_number {
        return ("Wrong node requested".to_string(), false, "".to_string());
    }
    let response = res["command"].as_str().unwrap();
    let error = res["error"].as_str().unwrap();
    let (response, isfile, file_path) = state_machine(response.to_string(), error.to_string());
    let mut final_response = String::new();
    final_response.push_str(response.as_str());
    (final_response, isfile, file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_run_deploy() {
        let command = "/deploy git@github.com:plswork/plswork.git".to_string();
        let expected = "lauda".to_string();
        let (result, isfile, filename) = state_machine(command, "".to_string());
        println!("isFile : {}", isfile);
        println!("filename : {}", filename);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_state_machine_run_command() {
        let command = "/run echo hello".to_string();
        let expected = "hello\n".to_string();
        let (result, _, _) = state_machine(command, "".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_state_machine_0() {
        let command = "/gaia start".to_string();
        let expected = "gaia start success".to_string();
        let (result, _, _) = state_machine(command, "".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_state_machine_0_1() {
        let command = "/gaia stop".to_string();
        let expected = "gaia stop success".to_string();
        let (result, _, _) = state_machine(command, "".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_state_machine_1() {
        let command = "/stats".to_string();
        let (result, _, _) = state_machine(command, "".to_string());
        assert!(result.contains("CPU"));
    }

    #[test]
    fn test_state_machine_2() {
        let command = "/error".to_string();
        let (result, _, _) = state_machine(command, "this is an error".to_string());
        assert_eq!(result, "error: this is an error".to_string());
    }

    #[test]
    fn test_string_to_json_1() {
        let inferred_json = String::from(
            "{\"node\":\"1\",\"command\":\"test command\",\"error\":\"this is an error\"}",
        );
        let expected = json!({
            "node": "1",
            "command": "test command",
            "error": "this is an error"
        });
        let result = string_to_json(inferred_json);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_string_to_json_2() {
        let inferred_json = String::from("{\"node\":\"1\",\"command\":\"test command\"}");
        let expected = json!({
            "node": "1",
            "command": "test command",
            "error": "Command Not Processed"
        });
        let result = string_to_json(inferred_json);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_string_to_json_3() {
        let inferred_json = String::from("{\"node\":\"1\"}");
        let expected = json!({
            "node": "1",
            "command": "",
            "error": "Command Not Processed"
        });
        let result = string_to_json(inferred_json);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_string_to_json_4() {
        let inferred_json =
            String::from("{\"command\":\"test command\",\"error\":\"this is an error\"}");
        let expected = json!({
            "node": "1",
            "command": "test command",
            "error": "this is an error"
        });
        let result = string_to_json(inferred_json);
        assert_eq!(result, expected);
    }
}
