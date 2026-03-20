fn main() {
    // push(char) で1文字末尾に追加
    let mut s1 = String::from("HelloWorld");
    s1.push('!');
    println!("{}", s1);

    // push_str(&str) で文字列スライスを末尾に追加
    let mut s2 = String::from("Hello");
    s2.push_str("World!");
    println!("{}", s2);

    // insert(index, char) で指定バイト位置に1文字を追加
    let mut s3 = String::from("HeloWorld!");
    s3.insert(3, 'l');
    println!("{}", s3);

    // insert_str(index, &str) で指定バイト位置に文字列スライスを追加
    let mut s4 = String::from("Helrld");
    s4.insert_str(3, "loW");
    println!("{}", s3);
}