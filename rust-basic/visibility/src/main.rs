// モジュール読み込み
mod module1; // ファイルで分割
mod module2; // ディレクトリでサブモジュールを管理する
mod module3; // mod.rs を使用する従来の方法

// use 宣言でモジュールをスコープに取り込む
use module1::submod::hello;
use module2::submod_1::hello as hello_module2_submod_1;

// ファイル内でのモジュール定義例
mod sample_mod {
    // 公開関数
    pub fn hello() {
        println!("[sample_mod] 公開関数");
        private_hello();
    }

    // 非公開関数
    fn private_hello() {
        println!("[sample_mod] 非公開関数");
    }
}

fn main() {
    println!("===== ファイル内でのモジュール定義例 =====");
    sample_mod::hello();
    // 以下の非公開関数の呼び出しはエラーになる
    // sample_mod::private_hello();

    println!("\n===== ファイルで分割する例 (module1) =====");
    module1::hello();
    // 以下の非公開関数の呼び出しはエラーになる
    // module1::private_hello();
    module1::submod::hello();
    module1::submod::nested::nested_hello();

    println!("\n===== ディレクトリでサブモジュールを管理する例 (module2) =====");
    module2::submod_1::hello();
    module2::submod_2::hello();

    println!("\n===== mod.rs を使用する従来の方法例 (module3) =====");
    module3::submod_1::hello();
    module3::submod_2::hello();

    println!("\n===== use 宣言でスコープに取り込む例 =====");
    hello();
    hello_module2_submod_1();
}
