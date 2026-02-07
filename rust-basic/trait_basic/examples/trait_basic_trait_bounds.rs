use std::f64::consts::PI;
use std::fmt::Debug;

// 領域に関するトレイト
trait Area {
    // 面積を計算するメソッド
    fn area(&self) -> f64;
}

// 円の構造体の定義
#[derive(Debug)]
struct Circle { radius: f64 }

// トレイトの実装 (Circle)
impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// 矩形の構造体の定義
#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }

// トレイトの実装 (Rectangle)
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// 面積を表示する関数 (トレイト制約)
fn show_area<T: Area + Debug>(shape: &T) {
    println!("{:?}, Area: {:.2}", shape, shape.area());
}

// // 面積を表示する関数 (トレイト制約を where 指定)
// fn show_area<T>(shape: &T)
// where T: Area + Debug
// {
//     println!("{:?}, Area: {:.2}", shape, shape.area());
// }

fn main() {
    let circle = Circle{
        radius: 5.0
    };
    show_area(&circle);

    let rect = Rectangle{
        width: 5.0,
        height: 10.5,
    };
    show_area(&rect);
}
