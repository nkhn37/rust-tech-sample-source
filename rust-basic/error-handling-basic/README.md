# エラー処理（Result型） (error-handling-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】エラー処理の基本を分かりやすく解説](https://rust-tech.nkhn37.net/rust-error-handling-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p error-handling-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p error-handling-basic
```

出力例：

```
cargo run -p error-handling-basic --example expect_option
cargo run -p error-handling-basic --example expect_result
cargo run -p error-handling-basic --example match_option
cargo run -p error-handling-basic --example match_result
cargo run -p error-handling-basic --example question_mark_operator_option
cargo run -p error-handling-basic --example question_mark_operator_option_main
cargo run -p error-handling-basic --example question_mark_operator_result
cargo run -p error-handling-basic --example question_mark_operator_result_main
cargo run -p error-handling-basic --example unwrap_option
cargo run -p error-handling-basic --example unwrap_result
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p error-handling-basic --example expect_option
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/error-handling-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example expect_option
cargo run --example expect_result
cargo run --example match_option
cargo run --example match_result
cargo run --example question_mark_operator_option
cargo run --example question_mark_operator_option_main
cargo run --example question_mark_operator_result
cargo run --example question_mark_operator_result_main
cargo run --example unwrap_option
cargo run --example unwrap_result
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example expect_option
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
