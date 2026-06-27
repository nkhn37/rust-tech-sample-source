# serde_json の基本 (serde-json-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust】serde で JSON を扱う基本を分かりやすく解説](https://rust-tech.nkhn37.net/rust-serde-json-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p serde-json-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p serde-json-basic
```

出力例：

```
cargo run -p serde-json-basic --example serde_json_deserialize
cargo run -p serde-json-basic --example serde_json_nested
cargo run -p serde-json-basic --example serde_json_option
cargo run -p serde-json-basic --example serde_json_option_skip
cargo run -p serde-json-basic --example serde_json_pretty
cargo run -p serde-json-basic --example serde_json_serialize
cargo run -p serde-json-basic --example serde_json_serialize_deserialize
cargo run -p serde-json-basic --example serde_json_vec
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p serde-json-basic --example serde_json_serialize
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-libraries/serde-json-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example serde_json_deserialize
cargo run --example serde_json_nested
cargo run --example serde_json_option
cargo run --example serde_json_option_skip
cargo run --example serde_json_pretty
cargo run --example serde_json_serialize
cargo run --example serde_json_serialize_deserialize
cargo run --example serde_json_vec
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example serde_json_serialize
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
