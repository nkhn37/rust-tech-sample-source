use std::error::Error;

fn parse_and_double(input: &str) -> Result<i32, Box<dyn Error>> {
    // 文字列を i32 に変換する（失敗すると ParseIntError が返る）
    let value: i32 = input.parse()?;

    if value == 0 {
        // 独自のエラーメッセージを Box<dyn Error> として返す
        return Err("値に 0 は指定できません".into());
    }

    Ok(value * 2)
}

fn main() {
    // 値を変更して試してみてください
    let inputs = ["21", "abc", "0"];

    for input in inputs {
        match parse_and_double(input) {
            Ok(result) => println!("result : {result}"),
            Err(e) => println!("error : {e}"),
        }
    }
}
