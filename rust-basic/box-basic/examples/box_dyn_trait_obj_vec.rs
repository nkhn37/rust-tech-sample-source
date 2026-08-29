trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    // 異なる型（Circle、Rectangle）でも、同じ Shape トレイトを実装していれば
    // Box<dyn Shape> として同じ Vec にまとめて格納できる
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 4.0,
        }),
    ];

    // 実行時に、それぞれの型の area メソッドが呼び分けられる（動的ディスパッチ）
    for shape in &shapes {
        let area = shape.area();
        println!("area : {area:.2}");
    }
}
