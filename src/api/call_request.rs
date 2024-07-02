/*
API response example:
{  
  "id": "chatcmpl-8mxZbodieGFDy6e3aZcY2UotnomBy",
  "object": "chat.completion",
  "created": 1706679435,
  "model": "gpt-3.5-turbo-0613",
  "choices": 
  [
      {
        "index": 0,
        "message": {
                    "role": "assistant\",
                    "content": "Hello! How can I assist you today?"      
                  },
        "logprobs": null,
        "finish_reason": "stop"
      }
  ],
    "usage": {
              "prompt_tokens": 9,
              "completion_tokens": 9,
              "total_tokens": 18
            },
    "system_fingerprint": null
}
*/


use crate::models::general::llm::{ Message, ChatCompletion, APIResponse };
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{ HeaderMap, HeaderValue };

// call large language model
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>>{
    dotenv().ok();

    //extract API key information
    let api_key: String = env::var("OPEN_AI_KEY").expect("key not found in env file");

    // confirm the url
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // create a header
    let mut headers: HeaderMap = HeaderMap::new();

    // add the API key
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?
    );

    // create the request
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature:0.1
    };

    // create client
    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error +Send> {Box::new(e)})?;

    // send the request
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?;

    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hello!".to_string()
        };

        let messages: Vec< Message> = vec!(message);

        let res = call_gpt(messages).await;
        match res {
            Ok(r) => {
                dbg!(r);
                assert!(true);
            },
            Err(_) => {
                dbg!("Error");
                assert!(false);
            }
        }
    }
}