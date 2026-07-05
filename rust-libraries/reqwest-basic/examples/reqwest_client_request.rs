use std::error::Error;
use std::time::Duration;

use reqwest::Method;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ClientBuilder でタイムアウトなどを設定する
    let client_builder = reqwest::Client::builder().timeout(Duration::from_secs(10));

    // ClientBuilder から Client を作成する
    let client = client_builder.build()?;

    // 対象の user_id
    let user_id = "u1001";

    // request() を使うと、get()・post()・put()・delete() のような専用メソッドの代わりに、
    // Method を指定して同じ内容のリクエストを送信できる
    let requests = [
        (
            "GET",
            Method::GET,
            "https://httpbin.org/anything/users".to_string(),
        ),
        (
            "POST",
            Method::POST,
            "https://httpbin.org/anything/users".to_string(),
        ),
        (
            "PUT",
            Method::PUT,
            format!("https://httpbin.org/anything/users/{user_id}"),
        ),
        (
            "DELETE",
            Method::DELETE,
            format!("https://httpbin.org/anything/users/{user_id}"),
        ),
    ];

    for (label, method, url) in requests {
        // RequestBuilder を作成する
        let request_builder = client.request(method, url);

        // レスポンスを取得する
        let response = request_builder.send().await?;

        // ステータスコードを取得する
        let status = response.status();
        println!("[{label}] {status}");
    }

    Ok(())
}
