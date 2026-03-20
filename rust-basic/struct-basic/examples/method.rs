struct Person {
    first_name: String,
    last_name: String,
    sex: String,
    age: u32,
    birthday: String,
}

impl Person {
    // あいさつをするメソッド
    fn greet(&self) {
        println!(
            "こんにちは。{}{}と言います。{}歳の{}で、誕生日は{}です。",
            self.last_name, self.first_name, self.age, self.sex, self.birthday,
        );
    }

    // 氏名を返却するメソッド
    fn full_name(&self) -> String {
        format!("{}{}", self.last_name, self.first_name)
    }

    // 年齢を比較するメソッド (self以外の引数をとる場合)
    fn is_older_than(&self, other_age: u32) -> bool {
        self.age > other_age
    }

    // 誕生日で年齢を+1するメソッド (可変参照とする場合)
    fn have_birthday(&mut self) {
        self.age += 1;
    }

    // 新しいPersonインスタンスを生成する (関連関数という)
    fn new(first_name: &str, last_name: &str, sex: &str, age: u32, birthday: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            sex: sex.to_string(),
            age,
            birthday: birthday.to_string(),
        }
    }
}

fn main() {
    // 関連関数を使って Person のインスタンスを作成
    let mut person = Person::new("太郎", "山田", "男性", 25, "2000-01-01");

    // greet メソッドの呼び出し
    person.greet();

    // full_name メソッドの呼び出し
    let full_name = person.full_name();
    println!("フルネーム: {}", full_name);

    // is_older_than メソッドの呼び出し
    let other_age = 30;
    if person.is_older_than(other_age) {
        println!("{}歳より年上です。", other_age);
    } else {
        println!("{}歳より年下です。", other_age);
    }

    // 誕生日を迎えたので have_birthday を呼び出し
    person.have_birthday();
    println!("誕生日を迎えて、{}歳になりました。", person.age);
}
