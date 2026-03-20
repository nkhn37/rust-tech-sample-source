# 列挙型の基本 (enum_basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】列挙型の基本を分かりやすく解説](https://rust-tech.nkhn37.net/rust-enum-basic/)
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

cargo run --example enum_basic
cargo run --example enum_option
cargo run --example enum_result
cargo run --example enum_values
```

表示されたコマンドを実行することで各サンプルプログラムを実行することができます。

> **補足**
>
> - `main.rs` で `common` プロジェクトの関数を使用しています。そのため本プロジェクト単体ではなく、`rust-tech-sample-source` をクローンして workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
