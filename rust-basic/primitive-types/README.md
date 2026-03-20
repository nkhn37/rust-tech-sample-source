# 基本型（スカラー型と複合型） (primitive-types)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】基本型を分かりやすく解説（スカラー型と複合型）](https://rust-tech.nkhn37.net/rust-basic-types/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p primitive-types` を指定して `cargo run` を実行します。

```bash
cargo run -p primitive-types
```

出力例：

```
cargo run -p primitive-types --example array
cargo run -p primitive-types --example boolean
cargo run -p primitive-types --example char
cargo run -p primitive-types --example floating_point
cargo run -p primitive-types --example integer
cargo run -p primitive-types --example string
cargo run -p primitive-types --example tuple
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p primitive-types --example array
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/primitive-types
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example array
cargo run --example boolean
cargo run --example char
cargo run --example floating_point
cargo run --example integer
cargo run --example string
cargo run --example tuple
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example array
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
