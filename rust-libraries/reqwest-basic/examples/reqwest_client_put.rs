use std::error::Error;
use std::time::Duration;

use serde::Serialize;

// リクエストボディとして送信するデータ
// user_id は URL のパスパラメータで指定するため、ボディには含めない
#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 更新対象の user_id
    let user_id = "u1001";

    // httpbin.org へ送信する
    // user_id をパスパラメータとして URL に含める
    let url = format!("https://httpbin.org/anything/users/{user_id}");
    // Docker を使用する場合は、以下のように URL を変更してください
    // let url = format!("http://localhost:8080/anything/users/{user_id}");

    // ClientBuilder でタイムアウトなどを設定する
    let client_builder = reqwest::Client::builder().timeout(Duration::from_secs(10));

    // ClientBuilder から Client を作成する
    let client = client_builder.build()?;

    // 更新後のデータ
    let user = User {
        name: "Miki".to_string(),
        age: 25,
    };

    // RequestBuilder を作成する
    // put() で PUT リクエストを送信する
    let request_builder = client.put(url).json(&user);

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
