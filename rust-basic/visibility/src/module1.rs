// module1.rs

// 公開関数
pub fn hello() {
    println!("[module1] 公開関数");
    private_hello();
    private_mod::hello();
}

// 非公開関数
fn private_hello() {
    println!("[module1] 非公開関数");
}

// 公開サブモジュール
pub mod submod {
    // サブモジュール内の公開関数
    pub fn hello() {
        println!("[module1::submod] サブモジュール内の公開関数");
        private_hello();
    }

    // サブモジュール内の非公開関数
    fn private_hello() {
        println!("[module1::submod] サブモジュール内の非公開関数");
    }

    // ネストされた公開サブモジュール
    pub mod nested {
        pub fn nested_hello() {
            println!("[module1::submod::nested] ネストされたサブモジュール内の公開関数");
        }
    }
}

// 非公開のサブモジュール
mod private_mod {
    pub fn hello() {
        println!("[module1::private_mod] 非公開のサブモジュール");
    }
}
