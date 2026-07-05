use std::error::Error;
use std::time::Duration;

// blocking::Client を使うと、async/await を使わずに同期的に通信できる
// tokio::main は不要で、通常の fn main で実行できる
fn main() -> Result<(), Box<dyn Error>> {
    // httpbin.org から取得する
    let url = "https://httpbin.org/get";
    // Docker を使用する場合は、以下のように URL を変更してください
    // let url = "http://localhost:8080/get";

    // ClientBuilder でタイムアウトなどを設定する
    let client_builder = reqwest::blocking::Client::builder().timeout(Duration::from_secs(10));

    // ClientBuilder から Client を作成する
    let client = client_builder.build()?;

    // 同じ Client で複数回リクエストを送る
    for count in 1..=3 {
        // RequestBuilder を作成する
        let request_builder = client.get(url);

        // レスポンスを取得する(await が不要)
        let response = request_builder.send()?;

        // ステータスコードを取得する
        let status = response.status();
        println!("{count}回目 ステータスコード: {status}");
    }

    Ok(())
}
