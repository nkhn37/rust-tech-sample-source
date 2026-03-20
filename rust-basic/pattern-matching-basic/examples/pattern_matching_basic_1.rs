// 曜日を表す列挙体
enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

// 曜日を表示する
fn show_day_of_week(weekday: &WeekDay) {
    // パターンマッチングで条件分岐
    match weekday {
        WeekDay::Sunday => println!("日曜日"),
        WeekDay::Monday => println!("月曜日"),
        WeekDay::Tuesday => println!("火曜日"),
        WeekDay::Wednesday => println!("水曜日"),
        WeekDay::Thursday => println!("木曜日"),
        WeekDay::Friday => println!("金曜日"),
        WeekDay::Saturday => println!("土曜日"),
    }
}

fn main() {
    let sunday = WeekDay::Sunday;
    show_day_of_week(&sunday);

    let monday = WeekDay::Monday;
    show_day_of_week(&monday);

    let tuesday = WeekDay::Tuesday;
    show_day_of_week(&tuesday);

    let wednesday = WeekDay::Wednesday;
    show_day_of_week(&wednesday);

    let thursday = WeekDay::Thursday;
    show_day_of_week(&thursday);

    let friday = WeekDay::Friday;
    show_day_of_week(&friday);

    let saturday = WeekDay::Saturday;
    show_day_of_week(&saturday);
}
