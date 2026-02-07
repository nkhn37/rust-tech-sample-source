fn main() {
    let s1 = "HelloWorld";
    let s2 = String::from("HelloWorld");

    let s1_part = &s1[2..5];
    let s2_part = &s2[5..];
    println!("s1_part: {}", s1_part);
    println!("s2_part: {}", s2_part);
}
