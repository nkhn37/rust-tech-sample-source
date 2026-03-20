use std::f64::consts::PI;

// 領域に関するトレイト
trait Area {
    // 面積を計算するメソッド
    fn area(&self) -> f64;
    // デフォルト実装
    fn describe(&self) {
        println!("Area: {:.2}", self.area())
    }
}

// 円の構造体の定義
struct Circle { radius: f64 }

// トレイトの実装 (Circle)
impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    // describe はデフォルト実装を使用
}

// 矩形の構造体の定義
struct Rectangle { width: f64, height: f64 }

// トレイトの実装 (Rectangle)
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    // describeのデフォルト実装をオーバーライドしている
    fn describe(&self) {
        println!("Rectangle(width={:.2}, height={:.2}), Area: {:.2}", self.width, self.height, self.area());
    }
}

fn main() {
    let circle = Circle{
        radius: 5.0
    };
    let _circle_area = circle.area();
    circle.describe();

    let rect = Rectangle{
        width: 5.0,
        height: 10.5,
    };
    let _rect_area = rect.area();
    rect.describe();
}
