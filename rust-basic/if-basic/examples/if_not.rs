fn main() {
    let is_logged_in = false;

    // 否定 (NOT)で条件を指定する
    if !is_logged_in {
        println!("ログインしてください。");
    } else {
        println!("ようこそ！");
    }
}
