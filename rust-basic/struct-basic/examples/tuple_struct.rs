// Color タプル構造体
struct Color(u8, u8, u8);
// Point タプル構造体
struct Point(i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0);

    println!("黒 = ({}, {}, {})", black.0, black.1, black.2);
    println!("原点 = ({}, {})", origin.0, origin.1);
}
