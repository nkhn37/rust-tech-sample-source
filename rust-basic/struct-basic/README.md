# 構造体の使い方 (struct-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust 入門】構造体の使い方を分かりやすく解説](https://rust-tech.nkhn37.net/rust-structs-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p struct-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p struct-basic
```

出力例：

```
cargo run -p struct-basic --example field_init
cargo run -p struct-basic --example field_init_shorthand
cargo run -p struct-basic --example method
cargo run -p struct-basic --example mutable_immutable
cargo run -p struct-basic --example ownership_borrowing
cargo run -p struct-basic --example person
cargo run -p struct-basic --example tuple_struct
cargo run -p struct-basic --example update_syntax
cargo run -p struct-basic --example update_syntax_move
cargo run -p struct-basic --example update_syntax_safe
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p struct-basic --example field_init
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/struct-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
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

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example field_init
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
