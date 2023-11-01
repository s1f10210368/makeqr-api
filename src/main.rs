use reqwest::Client;
use serde_json::json;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let api_key = "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";
    let endpoint = "https://api.openai.com/v1/chat/completions";

    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "system", "content": "こんにちは！"}]
    });

    let client = Client::new();
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let result = response.text().await?;

    let response_json: Value = serde_json::from_str(&result)?;

    // contentを取得
    if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
        println!("Content: {}", content);
    } else {
        println!("Content not found in the response.");
    }

    Ok(())
}




