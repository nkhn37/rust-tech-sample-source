# 条件分岐（if） (if-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】条件分岐 if を分かりやすく解説](https://rust-tech.nkhn37.net/rust-if-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p if-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p if-basic
```

出力例：

```
cargo run -p if-basic --example if_and
cargo run -p if-basic --example if_basic
cargo run -p if-basic --example if_expression
cargo run -p if-basic --example if_not
cargo run -p if-basic --example if_or
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p if-basic --example if_basic
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/if-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example if_and
cargo run --example if_basic
cargo run --example if_expression
cargo run --example if_not
cargo run --example if_or
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example if_basic
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
