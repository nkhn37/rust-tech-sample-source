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
    // header() でリクエストヘッダーを付与することができる
    // API キーを送信する場合によく使われる例
    let request_builder = client.get(url).header("X-API-Key", "sample-api-key");

    // レスポンスを取得する
    let response = request_builder.send().await?;

    // ステータスコードを取得する
    let status = response.status();
    println!("ステータスコード:\n{status}\n");

    // ヘッダーを取得する
    let headers = response.headers();
    println!("ヘッダー:\n{headers:?}\n");

    // ヘッダーから特定の項目だけを取り出す例
    // ヘッダーは HeaderMap 型で取得されるので、get() メソッドで取り出す
    // 値の型は HeaderValue なので、文字列に変換する必要がある
    let content_type = headers
        .get("content-type")
        .ok_or("content-type ヘッダーが見つかりません")?
        .to_str()?;
    println!("content-type ヘッダー: {content_type}\n");

    // レスポンスボディを文字列として取得する
    let text = response.text().await?;
    println!("応答の文字列形式:\n{text}");

    Ok(())
}
