fn main() {
    let mut s = String::from("HelloWorld");
    
    // pop() で末尾の文字を取得
    match s.pop() {
        Some(c) => println!("pop: {}", c),
        None => println!("文字列が空なので取り出せません"),
    }
    println!("s: {}", s);

    // remove(index) で指定バイト位置の1文字を削除
    let c = s.remove(1);
    println!("remove idx:{}, removed:{}", 1, c);
    println!("s: {}", s);

    // truncate で指定バイト位置以降を削除
    s.truncate(4);
    println!("s: {}", s);

    // 文字列を全て削除する
    s.clear();
    println!("s: {}", s);
}
