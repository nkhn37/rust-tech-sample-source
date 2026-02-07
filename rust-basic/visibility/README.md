# Rust の可視性 (visibility) のサンプルコード

## 概要

このプロジェクトは Rust Tech の記事  
[【Rust 入門】モジュールの可視性の仕組みを分かりやすく解説]()  
で紹介しているサンプルコードです。

## 実行方法

以下のコマンドで実行してください。

```bash
cargo run
```

## フォルダ構成の説明

### ファイル内でのモジュール定義

`main.rs` 内で `inline_mod` モジュールを定義しています。

```bash
src/
└─ main.rs
```

### ファイルでのモジュール定義

`file_mod.rs` でモジュールを定義しています。

```bash
src/
├─ main.rs
└─ file_mod.rs    ← 別ファイルモジュールの例
```

### ディレクトリでのモジュール定義

#### 一般的な定義例

親モジュール `dir_mod.rs` を定義し、`dir_mod` ディレクトリ配下に子モジュールを配置する方法です。(Rust 2018+ の定義方法)

```bash
src/
├─ main.rs
├─ dir_mod.rs    ← 親モジュール（2018+ の場合）
└─ dir_mod/
    └─ child.rs  ← 子モジュール
```

#### 従来の定義例

`legacy_dir_mod` 配下に `mod.rs` を定義し、子モジュールを配置する従来の定義方法

```bash
src/
├─ main.rs
└─ legacy_dir_mod/
    ├─ mod.rs    ← 従来方式
    └─ child.rs  ← 子モジュール
```
