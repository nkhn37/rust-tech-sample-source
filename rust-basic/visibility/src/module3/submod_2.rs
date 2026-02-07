// module3/submod_2.rs

// 公開関数
pub fn hello() {
    println!("[module3::submod_2] 公開関数");
    private_hello();
}

fn private_hello() {
    println!("[module3::submod_2] 非公開関数");
}
