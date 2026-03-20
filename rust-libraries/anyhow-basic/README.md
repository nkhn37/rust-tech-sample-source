# anyhow の基本 (anyhow-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust】anyhow で複数のエラーをまとめて扱う方法](https://rust-tech.nkhn37.net/rust-anyhow-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p anyhow-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p anyhow-basic
```

出力例：

```
cargo run -p anyhow-basic --example anyhow_anyhow_macro
cargo run -p anyhow-basic --example anyhow_bail_macro
cargo run -p anyhow-basic --example anyhow_basic
cargo run -p anyhow-basic --example anyhow_context
cargo run -p anyhow-basic --example anyhow_with_context
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p anyhow-basic --example anyhow_anyhow_macro
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-libraries/anyhow-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example anyhow_anyhow_macro
cargo run --example anyhow_bail_macro
cargo run --example anyhow_basic
cargo run --example anyhow_context
cargo run --example anyhow_with_context
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example anyhow_anyhow_macro
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
