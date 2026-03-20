// メッセージを表す列挙型
enum Message {
    Quit,
    Echo(String),
    Move(i32, i32),
    ChangeColor { r: u8, g: u8, b: u8, a: f32 },
}

// メッセージに対する処理を実施
fn action_for_message(message: &Message) {
    // 構造を含めたパターンマッチ
    match message {
        Message::Quit => println!("終了します。"),
        Message::Echo(s) => println!("出力: {}", s),
        Message::Move(x, y) => println!("移動: ({}, {})", x, y),
        Message::ChangeColor{r, g, b, a} =>println!("色 (r, g, b, a) = ({}, {}, {}, {})", r, g, b, a),
    }
}

fn main() {
    let message1 = Message::Quit;
    action_for_message(&message1);

    let message2 = Message::Echo(String::from("パターンマッチング"));
    action_for_message(&message2);

    let message3 = Message::Move(5, 10);
    action_for_message(&message3);

    let message4 = Message::ChangeColor{ r: 255, g: 255, b: 0, a: 0.5 };
    action_for_message(&message4);
}
