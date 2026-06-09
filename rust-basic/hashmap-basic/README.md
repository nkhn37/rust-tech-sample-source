# HashMap型の基本 (hashmap-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】HashMap 型の基本について分かりやすく解説](https://rust-tech.nkhn37.net/rust-hashmap-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p hashmap-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p hashmap-basic
```

出力例：

```
cargo run -p hashmap-basic --example hashmap_basic_collect
cargo run -p hashmap-basic --example hashmap_basic_new
cargo run -p hashmap-basic --example hashmap_for_basic
cargo run -p hashmap-basic --example hashmap_for_move
cargo run -p hashmap-basic --example hashmap_for_mut
cargo run -p hashmap-basic --example hashmap_operation_entryapi
cargo run -p hashmap-basic --example hashmap_operation_get
cargo run -p hashmap-basic --example hashmap_operation_insert
cargo run -p hashmap-basic --example hashmap_operation_remove
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p hashmap-basic --example hashmap_basic_collect
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/hashmap-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example hashmap_basic_collect
cargo run --example hashmap_basic_new
cargo run --example hashmap_for_basic
cargo run --example hashmap_for_move
cargo run --example hashmap_for_mut
cargo run --example hashmap_operation_entryapi
cargo run --example hashmap_operation_get
cargo run --example hashmap_operation_insert
cargo run --example hashmap_operation_remove
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example hashmap_basic_collect
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
