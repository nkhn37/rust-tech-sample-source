fn main() {
    let num = 2;

    // 偶数/奇数の判定をresultに設定する
    let result = if num % 2 == 0 { "偶数" } else { "奇数" };

    println!("{num}は、{result}です。");
}
