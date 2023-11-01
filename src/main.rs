use reqwest::Client;
use serde_json::json;
use serde_json::Value;
extern crate qrcode;
use qrcode::QrCode;
use qrcode::render::svg;

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

        let code = QrCode::new(content).unwrap();

        let image = code.render()
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

        std::fs::write("qrcode.svg", &image).expect("Unable to write QR code image");
    } else {
        println!("Content not found in the response.");
    }

    Ok(())
}




