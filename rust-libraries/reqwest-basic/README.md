# reqwest の基本 (reqwest-basic)

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust】reqwest で HTTP 通信をする基本を分かりやすく解説](https://rust-tech.nkhn37.net/rust-reqwest-basic/)
で紹介しているサンプルコードです。

## 実行方法

各サンプルコードは、`examples` フォルダ配下に配置してあります。  
`cargo run` を実行すると、サンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p reqwest-basic` を指定して `cargo run` を実行します。

```bash
cargo run -p reqwest-basic
```

出力例：

```
cargo run -p reqwest-basic --example reqwest_blocking
cargo run -p reqwest-basic --example reqwest_client_delete
cargo run -p reqwest-basic --example reqwest_client_get
cargo run -p reqwest-basic --example reqwest_client_get_header
cargo run -p reqwest-basic --example reqwest_client_get_json
cargo run -p reqwest-basic --example reqwest_client_get_query
cargo run -p reqwest-basic --example reqwest_client_post
cargo run -p reqwest-basic --example reqwest_client_put
cargo run -p reqwest-basic --example reqwest_client_request
cargo run -p reqwest-basic --example reqwest_error
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```bash
cargo run -p reqwest-basic --example reqwest_client_get
```

---

### 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```bash
cd rust-libraries/reqwest-basic
```

#### 2. `main.rs` を実行する

```bash
cargo run
```

出力例：

```
cargo run --example reqwest_blocking
cargo run --example reqwest_client_delete
cargo run --example reqwest_client_get
cargo run --example reqwest_client_get_header
cargo run --example reqwest_client_get_json
cargo run --example reqwest_client_get_query
cargo run --example reqwest_client_post
cargo run --example reqwest_client_put
cargo run --example reqwest_client_request
cargo run --example reqwest_error
```

#### 3. 表示されたコマンドを実行する

```bash
cargo run --example reqwest_client_get
```

> 補足
>
> - `main.rs` で `common` クレートの関数を使用しているため、本プロジェクト単体ではなく `rust-tech-sample-source` の workspace 配下で実行してください。
> - プログラムが変更・追加されている場合、上記と結果が異なる可能性があります。
