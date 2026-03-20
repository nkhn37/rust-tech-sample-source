#[allow(dead_code)]
// メッセージを表す列挙型
enum Message {
    Quit,
    Echo(String),
    Move(i32, i32),
    ChangeColor { r: u8, g: u8, b: u8, a: f32 },
}

fn move_action(message: &Message) {
    match message {
        Message::Move(x, y) if x == y => println!("対角方向への移動です: ({}, {})", x, y),
        Message::Move(x, y) => println!("移動先: ({}, {})", x, y),
        _ => (),
    }
}

fn main() {
    let message = Message::Move(5, 10);
    move_action(&message);

    let message = Message::Move(5, 5);
    move_action(&message);
}
