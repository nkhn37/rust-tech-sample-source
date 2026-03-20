# イテレータの基本 (iterator-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】イテレータ（Iterator）の基本を分かりやすく解説](https://rust-tech.nkhn37.net/rust-iterator-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p iterator-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p iterator-basic
```

出力例：

```
cargo run -p iterator-basic --example into_iter_basic
cargo run -p iterator-basic --example into_iter_for
cargo run -p iterator-basic --example iterator_basic
cargo run -p iterator-basic --example iterator_next
cargo run -p iterator-basic --example vec_iter
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p iterator-basic --example into_iter_basic
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/iterator-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example into_iter_basic
cargo run --example into_iter_for
cargo run --example iterator_basic
cargo run --example iterator_next
cargo run --example vec_iter
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example into_iter_basic
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
