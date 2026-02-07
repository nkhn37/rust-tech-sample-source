// メッセージを表す列挙型
enum Message {
    Quit,
    Echo(String),
    Move(i32, i32),
    ChangeColor { r: u8, g: u8, b: u8, a: f32 },
}

// メッセージに対する処理を実施
fn quit_only(message: &Message) {
    // 構造を含めたパターンマッチ
    match message {
        Message::Quit => println!("終了します。"),
        _ => (),  // Quit以外は何もしない
    }
}

fn main() {
    let message1 = Message::Quit;
    quit_only(&message1);

    let message2 = Message::Echo(String::from("パターンマッチング"));
    quit_only(&message2);

    let message3 = Message::Move(5, 10);
    quit_only(&message3);

    let message4 = Message::ChangeColor{ r: 255, g: 255, b: 0, a: 0.5 };
    quit_only(&message4);
}
