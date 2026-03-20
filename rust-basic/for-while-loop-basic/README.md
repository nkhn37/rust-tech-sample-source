# 繰り返し処理（for・while・loop） (for-while-loop-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】繰り返し処理（for, while, loop）を分かりやすく解説](https://rust-tech.nkhn37.net/rust-for-while-loop-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p for-while-loop-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p for-while-loop-basic
```

出力例：

```
cargo run -p for-while-loop-basic --example for_array
cargo run -p for-while-loop-basic --example for_basic
cargo run -p for-while-loop-basic --example for_break_continue
cargo run -p for-while-loop-basic --example for_vec_move
cargo run -p for-while-loop-basic --example for_vec_mut
cargo run -p for-while-loop-basic --example for_vec_ref
cargo run -p for-while-loop-basic --example loop_basic
cargo run -p for-while-loop-basic --example loop_break_return_value
cargo run -p for-while-loop-basic --example while_basic
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p for-while-loop-basic --example for_array
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/for-while-loop-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example for_array
cargo run --example for_basic
cargo run --example for_break_continue
cargo run --example for_vec_move
cargo run --example for_vec_mut
cargo run --example for_vec_ref
cargo run --example loop_basic
cargo run --example loop_break_return_value
cargo run --example while_basic
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example for_array
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
