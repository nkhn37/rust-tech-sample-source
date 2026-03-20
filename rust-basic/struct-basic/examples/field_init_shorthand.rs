struct Person {
    first_name: String,
    last_name: String,
    sex: String,
    age: u32,
    birthday: String,
}

fn create_person(
    first_name: String,
    last_name: String,
    sex: String,
    age: u32,
    birthday: String,
) -> Person {
    // フィールド初期化省略記法
    Person {
        first_name,
        last_name,
        sex,
        age,
        birthday,
    }
}

fn main() {
    let person = create_person(
        String::from("太郎"),
        String::from("山田"),
        String::from("男性"),
        25,
        String::from("2000-01-01"),
    );

    println!(
        "{}{}さん({})は、{}歳で誕生日は{}です。",
        person.last_name, person.first_name, person.sex, person.age, person.birthday
    );
}
