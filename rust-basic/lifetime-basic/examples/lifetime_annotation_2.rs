#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // // ==== この main はコンパイルエラーとなるため全体をコメントアウトしています。
    // // 試す際には全体のコメントを外して実行してみてください。

    // let s1 = String::from("Long String");
    // let result;

    // // 異なるスコープ
    // {
    //     let s2;

    //     // --- 以下は s2 の値の生存期間の前の借用 (エラーとなる)
    //     let ref_s2 = &s2;

    //     // --- s2 の値の生存期間が開始
    //     s2 = String::from("Short");

    //     // &s1 と &s2 は一時的な借用でライフタイムは終了、result のライフタイム開始
    //     result = longest(&s1, &s2);

    //     println!("{}", result);
    // } // s2 の値の生存期間終了

    // // 以下は result のライフタイム外なのでエラーとなる
    // println!("{}", result);
}
