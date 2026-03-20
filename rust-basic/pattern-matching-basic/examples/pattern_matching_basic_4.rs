// メッセージを表す列挙型
enum Message {
    Quit,
    Echo(String),
    Move(i32, i32),
    ChangeColor { r: u8, g: u8, b: u8, a: f32 },
}

// メッセージに対する処理を実施
fn action_for_message(message: &Message) -> u8 {
    // 構造を含めたパターンマッチ
    // match 式として値を返却する
    match message {
        Message::Quit => {
            println!("終了します。");
            0
        },
        Message::Echo(s) => {
            println!("出力: {}", s);
            1
        },
        Message::Move(x, y) => {
            println!("移動: ({}, {})", x, y);
            2
        },
        Message::ChangeColor{r, g, b, a} => {
            println!("色 (r, g, b, a) = ({}, {}, {}, {})", r, g, b, a);
            3
        },
    }
}

fn main() {
    // match の結果を code 変数に束縛する
    let message1 = Message::Quit;
    let code = action_for_message(&message1);
    println!("code: {}", code);

    let message2 = Message::Echo(String::from("パターンマッチング"));
    let code = action_for_message(&message2);
    println!("code: {}", code);

    let message3 = Message::Move(5, 10);
    let code = action_for_message(&message3);
    println!("code: {}", code);

    let message4 = Message::ChangeColor{ r: 255, g: 255, b: 0, a: 0.5 };
    let code = action_for_message(&message4);
    println!("code: {}", code);
}
