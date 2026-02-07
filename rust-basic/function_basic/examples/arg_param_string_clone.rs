// コピーを受け取っている
fn print_str_move(s: String) {
    println!("print_str_move: {}", s);
}

fn main() {
    // 所有権の移動の確認
    let s1 = String::from("Hello World");
    
    // 所有権が移動しないようにコピーを渡す
    print_str_move(s1.clone());
    println!("呼び出し元のs1: {}", s1);
}