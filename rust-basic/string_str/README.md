# String 型と文字列スライス &str 型の基本 (string_str)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】String 型と文字列スライス &str 型の基本について分かりやすく解説](https://rust-tech.nkhn37.net/rust-string-str-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
以下のコマンドで `main.rs` を実行すると、`examples` のプログラムを実行するためのコマンドが表示されます。本プロジェクトのルートで以下を実行してください。

```bash
cargo run
```

実行例：

```
例を実行するには、以下のコマンドを実行してください。

cargo run --example str_function
cargo run --example str_part_string
cargo run --example str_string_literal
cargo run --example str_structure
cargo run --example string_add
cargo run --example string_basic_format
cargo run --example string_basic_from
cargo run --example string_basic_new
cargo run --example string_basic_tostring
cargo run --example string_concat
cargo run --example string_index_panic
cargo run --example string_remove
cargo run --example string_remove_caution
cargo run --example string_structure
```

表示されたコマンドを実行することで各サンプルプログラムを実行することができます。

> **補足**
>
> - `main.rs` で `common` プロジェクトの関数を使用しています。そのため本プロジェクト単体ではなく、`rust-tech-sample-source` をクローンして workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
