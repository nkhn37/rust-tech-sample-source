#[allow(dead_code)]
#[derive(Debug)]
// メッセージを表す列挙型
enum Message {
    Quit,
    Echo(String),
    Move(i32, i32),
    ChangeColor { r: u8, g: u8, b: u8, a: f32 },
}

fn main() {
    // 終了を伝えるメッセージ
    let quit = Message::Quit;
    println!("Quit message: {quit:?}");

    // 文字列を表示するメッセージ
    let echo = Message::Echo(String::from("メッセージ"));
    println!("Echo message: {echo:?}");

    // (x, y) = (5, 10) への移動を表すメッセージ
    let move_position = Message::Move(5, 10);
    println!("Move message: {move_position:?}");

    // 色を変更する (r:赤, g:緑, b:青, a:透明度(0.0~1.0))
    let change_color = Message::ChangeColor {
        r: 255,
        g: 255,
        b: 0,
        a: 0.5,
    };
    println!("ChangeColor message: {change_color:?}");
}
