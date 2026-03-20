# 条件分岐 (if) の基本 (if_basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】条件分岐 if を分かりやすく解説](https://rust-tech.nkhn37.net/rust-if-basic/)
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

cargo run --example if_and
cargo run --example if_basic
cargo run --example if_expression
cargo run --example if_not
cargo run --example if_or
```

表示されたコマンドを実行することで各サンプルプログラムを実行することができます。

> **補足**
>
> - `main.rs` で `common` プロジェクトの関数を使用しています。そのため本プロジェクト単体ではなく、`rust-tech-sample-source` をクローンして workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
