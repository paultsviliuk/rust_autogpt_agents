use crate::models::general::llm::{Message, ChatCompletion};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// call large language model
pub async fn call_gpt(messages: Vec<Message>){
    dotenv.ok();

    //extract API key information
    let api_key: String = env::var("OPEN_AI_KEY").expect("key not found in env file");

    // confirm the url
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // create a header
    let mut headers: HeaderMap = HeaderMap.new();

    // add the API key
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap()
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
        .unwrap();

    // send the request
    let res_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
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

        call_gpt(messages).await;
    }
}