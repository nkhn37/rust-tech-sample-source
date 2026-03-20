# Vec型の基本 (vec-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】Vec 型の基本について分かりやすく解説](https://rust-tech.nkhn37.net/rust-vec-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p vec-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p vec-basic
```

出力例：

```
cargo run -p vec-basic --example vec_basic_macro
cargo run -p vec-basic --example vec_basic_macro_zeros
cargo run -p vec-basic --example vec_basic_new
cargo run -p vec-basic --example vec_for_move
cargo run -p vec-basic --example vec_for_move_into_iter
cargo run -p vec-basic --example vec_for_mut
cargo run -p vec-basic --example vec_for_mut_iter_mut
cargo run -p vec-basic --example vec_for_ref
cargo run -p vec-basic --example vec_for_ref_iter
cargo run -p vec-basic --example vec_method_append
cargo run -p vec-basic --example vec_method_clear
cargo run -p vec-basic --example vec_method_concat
cargo run -p vec-basic --example vec_method_extend
cargo run -p vec-basic --example vec_method_get
cargo run -p vec-basic --example vec_method_insert
cargo run -p vec-basic --example vec_method_pop
cargo run -p vec-basic --example vec_method_push
cargo run -p vec-basic --example vec_method_remove
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p vec-basic --example vec_basic_macro
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/vec-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example vec_basic_macro
cargo run --example vec_basic_macro_zeros
cargo run --example vec_basic_new
cargo run --example vec_for_move
cargo run --example vec_for_move_into_iter
cargo run --example vec_for_mut
cargo run --example vec_for_mut_iter_mut
cargo run --example vec_for_ref
cargo run --example vec_for_ref_iter
cargo run --example vec_method_append
cargo run --example vec_method_clear
cargo run --example vec_method_concat
cargo run --example vec_method_extend
cargo run --example vec_method_get
cargo run --example vec_method_insert
cargo run --example vec_method_pop
cargo run --example vec_method_push
cargo run --example vec_method_remove
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example vec_basic_macro
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
