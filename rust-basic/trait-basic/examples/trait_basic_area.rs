use std::f64::consts::PI;

// 領域に関するトレイト
trait Area {
    // 面積を計算するメソッド
    fn area(&self) -> f64;
}

// 円の構造体の定義
struct Circle { radius: f64 }

// トレイトの実装 (Circle)
impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// 矩形の構造体の定義
struct Rectangle { width: f64, height: f64 }

// トレイトの実装 (Rectangle)
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let circle = Circle{
        radius: 5.0
    };
    let circle_area = circle.area();
    println!("Circle(radius={:.2}), Area: {:.2}", circle.radius, circle_area);

    let rect = Rectangle{
        width: 5.0,
        height: 10.5,
    };
    let rect_area = rect.area();
    println!("Rectangle(width={:.2}, height={:.2}), Area: {:.2}", rect.width, rect.height, rect_area);
}
