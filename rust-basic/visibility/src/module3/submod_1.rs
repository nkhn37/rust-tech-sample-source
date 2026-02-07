// module3/submod_1.rs

// 公開関数
pub fn hello() {
    println!("[module3::submod_1] 公開関数");
    private_hello();
}

// 非公開関数
fn private_hello() {
    println!("[module3::submod_1] 非公開関数");
}
