// Person構造体を定義
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    sex: String,
    age: u32,
    birthday: String,
}

fn main() {
    // Personをインスタンスを作成する
    let person = Person {
        first_name: String::from("太郎"),
        last_name: String::from("山田"),
        sex: String::from("男性"),
        age: 25,
        birthday: String::from("2000-01-01"),
    };

    // 各要素にアクセスする
    println!(
        "{}{}さん({})は、{}歳で誕生日は{}です。",
        person.last_name, person.first_name, person.sex, person.age, person.birthday
    );
    // Debugフォーマットで表示する
    println!("{:?}", person);
}
