#[allow(dead_code)] // 未使用の警告を抑制
struct Person {
    first_name: String,
    last_name: String,
    sex: String,
    age: u32,
    birthday: String,
}

#[allow(dead_code)] // 未使用の警告を抑制
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
    let person1 = Person::new("太郎", "山田", "男性", 25, "2000-01-01");

    // 所有権が移動する
    let person2 = person1;

    // 以下はコメントアウトを外すとコンパイルエラー
    // println!("person1: {}", person1.first_name);

    // 所有権を移動させない場合は、参照を使用する
    let person3 = &person2;
    println!("person2: {}", person2.first_name);
    println!("person3: {}", person3.first_name);
}
