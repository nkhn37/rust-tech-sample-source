# イテレータの消費アダプタ (iterator-consuming-adapters)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】Iteratorの消費アダプタ（sum・fold・collect など）を分かりやすく解説](https://rust-tech.nkhn37.net/rust-iterator-consuming-adapters/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p iterator-consuming-adapters` を指定して `cargo run` を実行します。

```bash
cargo run -p iterator-consuming-adapters
```

出力例：

```
cargo run -p iterator-consuming-adapters --example collect_hashset
cargo run -p iterator-consuming-adapters --example collect_infinit_sequence
cargo run -p iterator-consuming-adapters --example collect_vec
cargo run -p iterator-consuming-adapters --example count_basic
cargo run -p iterator-consuming-adapters --example fold_basic
cargo run -p iterator-consuming-adapters --example fold_str_concat
cargo run -p iterator-consuming-adapters --example max_min_basic
cargo run -p iterator-consuming-adapters --example rfold_basic
cargo run -p iterator-consuming-adapters --example sum_product_basic
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p iterator-consuming-adapters --example collect_hashset
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/iterator-consuming-adapters
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example collect_hashset
cargo run --example collect_infinit_sequence
cargo run --example collect_vec
cargo run --example count_basic
cargo run --example fold_basic
cargo run --example fold_str_concat
cargo run --example max_min_basic
cargo run --example rfold_basic
cargo run --example sum_product_basic
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example collect_hashset
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
