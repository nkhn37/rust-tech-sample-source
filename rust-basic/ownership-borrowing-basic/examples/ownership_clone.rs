fn main() {
    let s1 = String::from("Hello World!");

    // clone によりコピーをすることも可能
    let s2 = s1.clone();

    // s1 も s2 もいずれも使える
    println!("{}", s1);
    println!("{}", s2);
}
