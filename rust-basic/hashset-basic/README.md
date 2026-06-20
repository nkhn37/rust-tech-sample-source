# HashSet型の基本 (hashset-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】HashSet 型の基本について分かりやすく解説](https://rust-tech.nkhn37.net/rust-hashset-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p hashset-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p hashset-basic
```

出力例：

```
cargo run -p hashset-basic --example hashset_basic_array_collect
cargo run -p hashset-basic --example hashset_basic_convert_to_vec
cargo run -p hashset-basic --example hashset_basic_new
cargo run -p hashset-basic --example hashset_basic_vec_collect
cargo run -p hashset-basic --example hashset_for_basic
cargo run -p hashset-basic --example hashset_for_move
cargo run -p hashset-basic --example hashset_operation_contains
cargo run -p hashset-basic --example hashset_operation_get
cargo run -p hashset-basic --example hashset_operation_insert
cargo run -p hashset-basic --example hashset_operation_len
cargo run -p hashset-basic --example hashset_operation_remove
cargo run -p hashset-basic --example hashset_relation_eq
cargo run -p hashset-basic --example hashset_relation_is_disjoint
cargo run -p hashset-basic --example hashset_relation_is_subset
cargo run -p hashset-basic --example hashset_relation_is_superset
cargo run -p hashset-basic --example hashset_set_operation_difference
cargo run -p hashset-basic --example hashset_set_operation_intersection
cargo run -p hashset-basic --example hashset_set_operation_symmetric_difference
cargo run -p hashset-basic --example hashset_set_operation_union
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p hashset-basic --example hashset_basic_new
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/hashset-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example hashset_basic_array_collect
cargo run --example hashset_basic_convert_to_vec
cargo run --example hashset_basic_new
cargo run --example hashset_basic_vec_collect
cargo run --example hashset_for_basic
cargo run --example hashset_for_move
cargo run --example hashset_operation_contains
cargo run --example hashset_operation_get
cargo run --example hashset_operation_insert
cargo run --example hashset_operation_len
cargo run --example hashset_operation_remove
cargo run --example hashset_relation_eq
cargo run --example hashset_relation_is_disjoint
cargo run --example hashset_relation_is_subset
cargo run --example hashset_relation_is_superset
cargo run --example hashset_set_operation_difference
cargo run --example hashset_set_operation_intersection
cargo run --example hashset_set_operation_symmetric_difference
cargo run --example hashset_set_operation_union
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example hashset_basic_new
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
