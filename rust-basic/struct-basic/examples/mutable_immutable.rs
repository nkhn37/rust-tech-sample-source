struct Person {
    first_name: String,
    last_name: String,
    sex: String,
    age: u32,
    birthday: String,
}

fn main() {
    // 可変 (mutable) で変数を定義
    let mut person1 = Person {
        first_name: String::from("太郎"),
        last_name: String::from("山田"),
        sex: String::from("男性"),
        age: 24,
        birthday: String::from("2000-01-01"),
    };
    // 値の変更はOK
    person1.age += 1;
    println!(
        "{} {} {} {} {}",
        person1.last_name, person1.first_name, person1.sex, person1.age, person1.birthday
    );

    // 不変 (immutable) で変数を定義
    let person2 = Person {
        first_name: String::from("花子"),
        last_name: String::from("山田"),
        sex: String::from("女性"),
        age: 20,
        birthday: String::from("2005-01-01"),
    };
    println!(
        "{} {} {} {} {}",
        person2.last_name, person2.first_name, person2.sex, person2.age, person2.birthday
    );

    // 値の変更はできない (コンパイルエラー)
    // person2.age += 1;
}
