use crate::models::general::llm::{ Message, OllamaRequest, APIResponse };
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

use reqwest::header::{ HeaderMap, HeaderValue };

pub async fn call_llm(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let model: String = env::var("MODEL").expect("MODEL not found in environment variables");
    let url: &str = "http://localhost:11434/api/generate";

    let prompt = messages
        .iter()
        .map(|msg| format!("{}: {}", msg.role, msg.content))
        .collect::<Vec<String>>()
        .join("\n");

    let client = Client::new();
    
    let request = OllamaRequest {
        model: model,
        prompt,
        stream: false,
    };

    let res: APIResponse = client
        .post(url)
        .json(&request)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // let ollama_response: OllamaResponse = response.json().await?;
    // Ok(ollama_response.response)
    Ok(res.response.clone())
}

#[cfg(test)]
mod tests {
    use super:: *;

    #[tokio::test]
    async fn tests_call_to_model() {
        let message: Message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test, give me a short response.".to_string()
        };

        let messages: Vec<Message> = vec!(message);
        let res: Result<String, Box<dyn std::error::Error + Send>> = call_llm(messages).await;
        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            },
            Err(err) => {
                dbg!(err);
                assert!(false);
            }
        }
    }
}
