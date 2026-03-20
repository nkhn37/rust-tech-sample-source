# 構造体の使い方 (struct_basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust 入門】構造体の使い方を分かりやすく解説](https://rust-tech.nkhn37.net/rust-structs-basic/)  
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

cargo run --example field_init
cargo run --example field_init_shorthand
cargo run --example method
cargo run --example mutable_immutable
cargo run --example ownership_borrowing
cargo run --example person
cargo run --example tuple_struct
cargo run --example update_syntax
cargo run --example update_syntax_move
cargo run --example update_syntax_safe
```

表示されたコマンドを実行することで各サンプルプログラムを実行することができます。

> **補足**
>
> - `main.rs` で `common` プロジェクトの関数を使用しています。そのため本プロジェクト単体ではなく、`rust-tech-sample-source` をクローンして workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
