use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

use serde::Deserialize;

// url と headers だけを保持する構造体
#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    url: String,
    headers: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // httpbin.org から取得する
    let url = "https://httpbin.org/get";
    // Docker を使用する場合は、以下のように URL を変更してください
    // let url = "http://localhost:8080/get";

    // ClientBuilder でタイムアウトなどを設定する
    let client_builder = reqwest::Client::builder().timeout(Duration::from_secs(10));

    // ClientBuilder から Client を作成する
    let client = client_builder.build()?;

    // RequestBuilder を作成する
    let request_builder = client.get(url);

    // レスポンスを取得する
    let response = request_builder.send().await?;

    // json() で構造体にデシリアライズする
    let data: HttpBinResponse = response.json().await?;

    // url と headers を取り出す
    println!("url: {}", data.url);
    println!("headers: {:?}", data.headers);

    Ok(())
}
