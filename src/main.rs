use reqwest::Client;
use serde_json::json;
use serde_json::Value;
extern crate qrcode;
extern crate encoding_rs;
use encoding_rs::SHIFT_JIS;
use qrcode::QrCode;
use qrcode::render::svg;
/*use qr2term::print_qr;*/

//以下はCHATGPTを使用する際の定型
#[tokio::main]  // これをつけると非同期関数が使える
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // OpenAI APIの認証キーとエンドポイントを設定
    let api_key = "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";
    let endpoint = "https://api.openai.com/v1/chat/completions";

    // APIリクエストのボディをJSON形式で設定する
    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "system", "content": "こんにちは!"}]
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
        /*
        // utf-8にエンコード
        // 日本語QRコード読み取りはShift-JISの方が適している
        let utf8_content = content.as_bytes();
        */
        // Shift-JISにエンコード
        let (content_encoded, _, had_error) = SHIFT_JIS.encode(&content);

        if had_error {
            println!("エンコード中にエラーが発生");
            return Err("エンコードエラー:".into());
        }

        // QRコードを生成
        let code = QrCode::new(&content_encoded).unwrap();

        // QRコードをSVG形式でレンダリング
        let image = code.render()
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();

        // ターミナルに出力するときは以下を用いる
        /*let qr_term = code.render()
            .dark_color('　')
            .light_color('■')
            .build();
        */

        // SVG形式のQRコードをファイルに保存
        //std::fs::write("qrcode.svg", image).expect("Unable to write QR code image");
        // SVG形式のQRコードをファイルに保存（バイナリエンコーディング）
        std::fs::write("qrcode.svg", &image).expect("Unable to write QR code image");


        // ターミナルに表示
        /*println!("{}", qr_term);*/
    } else {
        println!("Content not found in the response.");
    }

    Ok(())
}





