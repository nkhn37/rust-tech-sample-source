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
}
