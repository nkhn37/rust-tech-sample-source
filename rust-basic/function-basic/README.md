# 関数の基本 (function-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】関数の基本を分かりやすく解説](https://rust-tech.nkhn37.net/rust-function-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p function-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p function-basic
```

出力例：

```
cargo run -p function-basic --example arg_param_basic
cargo run -p function-basic --example arg_param_mutable
cargo run -p function-basic --example arg_param_string_clone
cargo run -p function-basic --example arg_param_string_move
cargo run -p function-basic --example arg_param_string_ref_change
cargo run -p function-basic --example function_basic
cargo run -p function-basic --example function_shadowing
cargo run -p function-basic --example multiple_results
cargo run -p function-basic --example return_value
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p function-basic --example arg_param_basic
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/function-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example arg_param_basic
cargo run --example arg_param_mutable
cargo run --example arg_param_string_clone
cargo run --example arg_param_string_move
cargo run --example arg_param_string_ref_change
cargo run --example function_basic
cargo run --example function_shadowing
cargo run --example multiple_results
cargo run --example return_value
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example arg_param_basic
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
