# rust-tech-sample-source

このリポジトリは、Rust Tech (https://rust-tech.nkhn37.net/) にて公開しているプログラムソースを含んでいます。

## 🦀 概要

このリポジトリでは、Rust 初心者向けの参考プログラムおよび応用プログラムを公開していきます。  
各トピックごとに Cargo プロジェクトとして作成しており、フォルダ配下にプログラムを配置しています。

## 🚀 プログラムの使い方

`cargo run` を実行すると、各プロジェクトの `examples` フォルダ内のサンプルコードの実行方法が表示されます。  
**リポジトリルートから実行する方法**と**各プロジェクトフォルダから実行する方法**のどちらにも対応しています。

---

### ▶ リポジトリルートから実行する場合

#### 1. `main.rs` を実行する

`-p <パッケージ名>` を指定して `cargo run` を実行します。

（例：`variables-and-constants` パッケージのサンプル一覧を表示する場合）

```
cargo run -p variables-and-constants
```

出力例：

```
cargo run -p variables-and-constants --example constant
cargo run -p variables-and-constants --example immutable
cargo run -p variables-and-constants --example mutable
cargo run -p variables-and-constants --example shadowing
cargo run -p variables-and-constants --example shadowing_other_scope
```

#### 2. 表示されたコマンドを実行する

表示されたコマンドをそのまま実行すると、サンプルプログラムを試すことができます。

```
cargo run -p variables-and-constants --example constant
```

---

### ▶ 各プロジェクトフォルダから実行する場合

#### 1. プロジェクトフォルダへ移動する

```
cd rust-basic/variables-and-constants
```

#### 2. `main.rs` を実行する

```
cargo run
```

出力例：

```
cargo run --example constant
cargo run --example immutable
cargo run --example mutable
cargo run --example shadowing
cargo run --example shadowing_other_scope
```

#### 3. 表示されたコマンドを実行する

```
cargo run --example constant
```

## 📁 ディレクトリ構成概要

```
rust-tech-sample-source/          # プロジェクトルート
├─ Cargo.toml                       # ワークスペース定義
├─ common/                          # 共通ライブラリクレート
├─ rust-basic/                      # Rust 入門
│  ├─ function-basic/                 # 関数の基本
│  ├─ struct-basic/                   # 構造体の基本
│  ├─ ownership-borrowing-basic/       # 所有権と借用
│  └─ ...                             # 各トピックのフォルダ
├─ rust-libraries/                  # Rust ライブラリ関連
│  ├─ anyhow-basic/                   # anyhow の基本
│  ├─ thiserror-basic/                # thiserror の基本
│  └─ ...                             # 各トピックのフォルダ
└─ README.md
```

## 💻 環境

- Rust 1.90 以降 (推奨)
- Cargo (Rust に同梱)
- 推奨エディタ：VS Code または RustRover

## 📜 ライセンス

このリポジトリに含まれる全てのコードは[MIT LICENSE](/LICENSE)に従います。  
オープンソースライセンス詳細は[Open Source Initiative](https://opensource.org/licenses/MIT)
を参照してください。
