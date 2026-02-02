fn main() {
    let result: Result<i32, &str> = Ok(42);
    println!("{result:?}");

    let error: Result<i32, &str> = Err("処理に失敗しました");
    println!("{error:?}");
}
