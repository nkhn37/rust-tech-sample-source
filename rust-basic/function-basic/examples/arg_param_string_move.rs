// 所有権が移動 (move) する場合
fn print_str_move(s: String) {
    println!("print_str_move: {}", s);
}

fn main() {
    // 所有権の移動の確認
    let s1 = String::from("Hello World");
    print_str_move(s1);

    // 呼び出し元のs1は所有権が移動しているため、使用できない
    // println!("呼び出し元のs1: {}", s1);
}
