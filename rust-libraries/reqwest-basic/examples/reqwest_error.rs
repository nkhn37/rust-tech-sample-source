use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // httpbin.org から取得する
    let url = "https://httpbin.org/status/400";
    // Docker を使用する場合は、以下のように URL を変更してください
    // let url = "http://localhost:8080/status/400";

    // ClientBuilder でタイムアウトなどを設定する
    let client_builder = reqwest::Client::builder().timeout(Duration::from_secs(10));

    // ClientBuilder から Client を作成する
    let client = client_builder.build()?;

    // ケース1: HTTP レベルのエラー(ステータスコードが 4xx や 5xx)
    // 通信自体は成功しているため、send() は Ok(Response) を返す
    let response = client.get(url).send().await?;
    println!("ステータスコード: {}\n", response.status());

    // error_for_status() を使うと、4xx・5xx のステータスコードを Err に変換できる
    match response.error_for_status() {
        Ok(response) => println!("成功として扱われました: {}", response.status()),
        Err(error) => println!("error_for_status() でエラーになりました: {error}"),
    }
    println!();

    // ケース2: 通信レベルのエラー
    // 存在しないホスト名のため、DNS 解決に失敗して send() 自体が Err を返す
    let invalid_url = "https://this-host-does-not-exist.com";
    match client.get(invalid_url).send().await {
        Ok(response) => println!("ステータスコード: {}", response.status()),
        Err(error) => println!("send() でエラーになりました: {error}"),
    }

    Ok(())
}
