// コンパイル時にエラーとなる例
// コンパイラは、List 型のサイズを決定できないため、コンパイルエラーとなる
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// use List::{Cons, Nil};

fn main() {
    println!("上記List定義部分と以下のコメントを外すとコンパイルエラーになる。");
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
}
