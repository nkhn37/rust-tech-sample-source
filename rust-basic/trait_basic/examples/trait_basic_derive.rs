// 円の構造体の定義
#[derive(Debug)]
struct Circle { radius: f64 }

// 矩形の構造体の定義
#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }

fn main() {
    let circle = Circle{
        radius: 5.0
    };
    println!("radius {}", circle.radius);
    println!("{:?}", circle);

    let rect = Rectangle{
        width: 5.0,
        height: 10.5,
    };
    println!("width {}, height {}", rect.width, rect.height);
    println!("{:?}", rect);
}
