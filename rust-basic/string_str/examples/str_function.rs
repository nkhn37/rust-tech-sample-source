fn show_str(s: &str) {
    println!("{}", s);
}

fn main() {
    let s1 = "HelloWorld";
    let s2 = String::from("HelloWorld");

    show_str(&s1);
    show_str(&s2);
}
