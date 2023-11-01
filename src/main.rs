use reqwest::Client;
use serde_json::json;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let api_key = "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";
    let endpoint = "https://api.openai.com/v1/chat/completions";

    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "who are you?"}]
    });

    let client = Client::new();
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let result = response.text().await?;
    
    println!("Response: {}", result);

    Ok(())
}




