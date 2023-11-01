use reqwest::Client;
use serde_json::json;
use serde_json::Value;
extern crate qrcode;
use qrcode::QrCode;
use qrcode::render::svg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // OpenAI APIの認証キーとエンドポイントを設定
    let api_key = "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";
    let endpoint = "https://api.openai.com/v1/chat/completions";

    // APIリクエストのボディをJSON形式で設定
    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "system", "content": "Unityについて教えてください"}]
    });

    // HTTPクライアントのインスタンスを作成
    let client = Client::new();

    // APIリクエストを送信し、レスポンスを取得
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    // レスポンスのテキストを取得
    let result = response.text().await?;

    // レスポンスをJSONにパース
    let response_json: Value = serde_json::from_str(&result)?;

    // レスポンスから"content"を取得
    if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
        println!("Content: {}", content);

        // QRコードを生成
        let code = QrCode::new(content).unwrap();

        // QRコードをSVG形式でレンダリング
        let image = code.render()
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();

        // SVG形式のQRコードをファイルに保存
        std::fs::write("qrcode.svg", &image).expect("Unable to write QR code image");
    } else {
        println!("Content not found in the response.");
    }

    Ok()
}





