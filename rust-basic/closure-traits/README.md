# クロージャとトレイト境界 (closure-traits)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】クロージャの型とトレイト（Fn / FnMut / FnOnce）](https://rust-tech.nkhn37.net/rust-closure-fn-traits/)
に関連するサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p closure-traits` を指定して `cargo run` を実行します。

```bash
cargo run -p closure-traits
```

出力例：

```
cargo run -p closure-traits --example closure_basic
cargo run -p closure-traits --example closure_fn_difference
cargo run -p closure-traits --example closure_trait_bounds
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p closure-traits --example closure_basic
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/closure-traits
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example closure_basic
cargo run --example closure_fn_difference
cargo run --example closure_trait_bounds
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example closure_basic
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
