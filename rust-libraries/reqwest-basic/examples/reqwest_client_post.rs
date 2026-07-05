use std::error::Error;
use std::time::Duration;

use serde::Serialize;

// リクエストボディとして送信するデータ
// user_id はサーバー側で採番される想定とする
#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // httpbin.org へ送信する
    // ユーザー登録エンドポイントを呼び出す
    let url = "https://httpbin.org/anything/users";
    // Docker を使用する場合は、以下のように URL を変更してください
    // let url = "http://localhost:8080/anything/users";

    // ClientBuilder でタイムアウトなどを設定する
    let client_builder = reqwest::Client::builder().timeout(Duration::from_secs(10));

    // ClientBuilder から Client を作成する
    let client = client_builder.build()?;

    // 送信するデータ
    let user = User {
        name: "Taro".to_string(),
        age: 30,
    };

    // RequestBuilder を作成する
    // json() でリクエストボディを JSON として送信する
    let request_builder = client.post(url).json(&user);

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
