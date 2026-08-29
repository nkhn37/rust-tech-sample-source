// Rust Book でも紹介されている連結リストの例
// https://rust-book.cs.brown.edu/ch15-01-box.html
// 自分自身（List）を参照する再帰的な型は、コンパイル時にサイズが決まらないため、コンパイルエラーとなる
// そのため、Box スタック上にはサイズの決まったポインタのみを置き、ヒープ上に List の値を置くことで定義する
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

// List の中身を表示する関数
fn print_list(list: &List) {
    match list {
        // List の中身を表示する
        Cons(value, next) => {
            println!("{value}");
            print_list(next);
        }
        // 末端の Nil の場合は、Nil と表示する
        Nil => println!("Nil"),
    }
}
fn main() {
    // 1 -> 2 -> 3 -> Nil という連結リストを表す
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    print_list(&list);
}
