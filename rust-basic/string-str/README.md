# String型と&strの違い (string-str)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】String 型と文字列スライス &str 型の基本について分かりやすく解説](https://rust-tech.nkhn37.net/rust-string-str-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p string-str` を指定して `cargo run` を実行します。

```bash
cargo run -p string-str
```

出力例：

```
cargo run -p string-str --example str_function
cargo run -p string-str --example str_part_string
cargo run -p string-str --example str_string_literal
cargo run -p string-str --example str_structure
cargo run -p string-str --example string_add
cargo run -p string-str --example string_basic_format
cargo run -p string-str --example string_basic_from
cargo run -p string-str --example string_basic_new
cargo run -p string-str --example string_basic_tostring
cargo run -p string-str --example string_concat
cargo run -p string-str --example string_index_panic
cargo run -p string-str --example string_remove
cargo run -p string-str --example string_remove_caution
cargo run -p string-str --example string_structure
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p string-str --example str_function
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/string-str
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
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

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example str_function
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
