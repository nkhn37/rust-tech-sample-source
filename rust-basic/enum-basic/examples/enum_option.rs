fn main() {
    let name: Option<String> = Some(String::from("山田太郎"));
    println!("{name:?}");

    let empty: Option<String> = None;
    println!("{empty:?}");
}
