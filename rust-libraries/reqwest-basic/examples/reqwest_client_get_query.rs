use std::error::Error;
use std::time::Duration;

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
    // query() でクエリパラメータを付与することができる
    let queries = [("key1", "value1"), ("key2", "value2")];
    let request_builder = client.get(url).query(&queries);

    // レスポンスを取得する
    let response = request_builder.send().await?;

    // ステータスコードを取得する
    let status = response.status();
    println!("ステータスコード:\n{status}\n");

    // レスポンスボディを文字列として取得する
    let text = response.text().await?;
    println!("応答の文字列形式:\n{text}");

    Ok(())
}
