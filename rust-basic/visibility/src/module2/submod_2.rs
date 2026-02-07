// module2/submod_2.rs

// 公開関数
pub fn hello() {
    println!("[module2::submod_2] 公開関数");
    private_hello();
}

fn private_hello() {
    println!("[module2::submod_2] 非公開関数");
}
