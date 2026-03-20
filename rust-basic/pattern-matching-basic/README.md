# パターンマッチング (pattern-matching-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust入門】パターンマッチングの基本について分かりやすく解説](https://rust-tech.nkhn37.net/rust-pattern-matching-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p pattern-matching-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p pattern-matching-basic
```

出力例：

```
cargo run -p pattern-matching-basic --example option_example
cargo run -p pattern-matching-basic --example pattern_matching_basic
cargo run -p pattern-matching-basic --example pattern_matching_basic_1
cargo run -p pattern-matching-basic --example pattern_matching_basic_2
cargo run -p pattern-matching-basic --example pattern_matching_basic_3
cargo run -p pattern-matching-basic --example pattern_matching_basic_4
cargo run -p pattern-matching-basic --example pattern_matching_if_let
cargo run -p pattern-matching-basic --example pattern_matching_match_guard
cargo run -p pattern-matching-basic --example result_example
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p pattern-matching-basic --example option_example
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-basic/pattern-matching-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example option_example
cargo run --example pattern_matching_basic
cargo run --example pattern_matching_basic_1
cargo run --example pattern_matching_basic_2
cargo run --example pattern_matching_basic_3
cargo run --example pattern_matching_basic_4
cargo run --example pattern_matching_if_let
cargo run --example pattern_matching_match_guard
cargo run --example result_example
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example option_example
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
