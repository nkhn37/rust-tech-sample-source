#[allow(dead_code)]
#[derive(Debug)]
// 曜日を表す列挙型
enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn main() {
    let today = WeekDay::Sunday;
    println!("Today is {today:?}");
}
