#[allow(dead_code)]
// メッセージを表す列挙型
enum Message {
    Quit,
    Echo(String),
    Move(i32, i32),
    ChangeColor { r: u8, g: u8, b: u8, a: f32 },
}

// メッセージに対する処理を実施
fn echo_message(message: &Message) {
    // if let を使用して Echo(s) パターンのみにマッチング
    if let Message::Echo(s) = message {
        println!("出力: {}", s);
    }
}

fn main() {
    let message1 = Message::Quit;
    echo_message(&message1);

    let message2 = Message::Echo(String::from("パターンマッチング"));
    echo_message(&message2);

    let message3 = Message::Move(5, 10);
    echo_message(&message3);

    let message4 = Message::ChangeColor {
        r: 255,
        g: 255,
        b: 0,
        a: 0.5,
    };
    echo_message(&message4);
}
