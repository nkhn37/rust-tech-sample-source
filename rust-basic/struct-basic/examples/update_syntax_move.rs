#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    sex: String,
    age: u32,
    birthday: String,
}

fn main() {
    let person1 = Person {
        first_name: String::from("太郎"),
        last_name: String::from("山田"),
        sex: String::from("男性"),
        age: 25,
        birthday: String::from("2000-01-01"),
    };

    // 構造体更新記法で新しいインスタンスを生成する
    let person2 = Person {
        first_name: String::from("花子"),
        sex: String::from("女性"),
        ..person1
    };

    println!(
        "{}{}さん({})は、{}歳で誕生日は{}です。",
        person2.last_name, person2.first_name, person2.sex, person2.age, person2.birthday
    );

    // 一部フィールドでも所有権の移動が起こると構造体本体を使用できない。
    // ただし、個々のフィールドについては Copy トレイトの実装有無で挙動が異なる
    //  - Copy トレイトが実装されている場合: フィールドの値がコピーされるので使用できる
    //  - Copy トレイトが実装されていない場合: フィールドの所有権が移動するので使用不可

    // 以下のコメントアウトを外すとコンパイルエラー
    // println!("{:?}", person1);
    println!("{}", person1.first_name);
    // println!("{}", person1.last_name);
    println!("{}", person1.sex);
    println!("{}", person1.age);
    // println!("{}", person1.birthday);
}
