// 矩形構造体 Rectangle
struct Rectangle {
    width: u32,
    height: u32,
    color_code: u8,
}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 50,
        color_code: 1,
    };

    // 構造体更新記法で新しいインスタンスを生成する
    // 基本型ばかりなのでコピーになる
    let rect2 = Rectangle {
        color_code: 2,
        ..rect1
    };

    // rect1 のフィールドにもアクセスできる
    println!(
        "矩形1 widhth:{}, height:{}, color_code:{}",
        rect1.width, rect1.height, rect1.color_code
    );
    println!(
        "矩形2 widhth:{}, height:{}, color_code:{}",
        rect2.width, rect2.height, rect2.color_code
    );
}
