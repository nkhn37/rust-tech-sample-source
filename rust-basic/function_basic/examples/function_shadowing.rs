fn message() {
    println!("メッセージ 1");
}

fn show_message() {
    message();  // 「メッセージ 1」を表示
}

fn main() {
    message();  // 「メッセージ 2」を表示
    {
        message();  // 「メッセージ 3」を表示

        fn message() {
            println!("メッセージ 3");
        }
    }
    message();  // 「メッセージ 2」を表示
    show_message(); // show_message関数経由で「メッセージ 1」を表示

    fn message() {
        println!("メッセージ 2");
    }
}
