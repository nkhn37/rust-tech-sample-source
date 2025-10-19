# rust-tech-sample-source

このリポジトリは、Rust Tech (https://rust-tech.nkhn37.net/) にて公開しているプログラムソースを含んでいます。

## 🦀 概要

このリポジトリでは、Rust 初心者向けの参考プログラムおよび応用プログラムを公開していきます。  
各トピックごとに Cargo プロジェクトとして作成しており、フォルダ配下にプログラムを配置しています。

## 🚀 プログラムの使い方

### 1. プロジェクトフォルダへ移動

実行したいトピックのフォルダへ移動します。

（例：`rust-basic/variables_and_constants` フォルダのプログラムを試す場合）

```
cd rust-basic/variables_and_constants
```

### 2. `main.rs` を実行する

各トピックのプロジェクトフォルダで`cargo run` により `main.rs` を実行します。  
このプログラムは、各プロジェクトフォルダの `examples` フォルダ内のサンプルコードの実行方法を表示します。

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

### 3. 表示されたコマンドを実行する

表示されたコマンドを実行すると、ブログ記事内で紹介しているサンプルプログラムを、そのまま試すことができます。

```
cargo run --example constant
```

## 📁 ディレクトリ構成概要

```
rust-tech-sample-source/
├─ Cargo.toml             # ワークスペース定義
├─ common/                # 共通ライブラリクレート
├─ rust-basic/
│  ├─ function_basic/     # 関数の基本
│  ├─ struct_basic/       # 構造体の基本
│  ├─ ownership_borrowing_basic/  # 所有権と借用
│  └─ ...                 # 各トピックのフォルダ
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
