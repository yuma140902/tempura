# Tempura

[![](https://img.shields.io/github/actions/workflow/status/yuma140902/tempura/ci.yml?logo=linux&logoColor=white&label=CI%20on%20Linux)](https://github.com/yuma140902/tempura/actions)
[![](https://img.shields.io/github/actions/workflow/status/yuma140902/tempura/ci.yml?logo=windows&logoColor=white&label=CI%20on%20Windows)](https://github.com/yuma140902/tempura/actions)
[![](https://img.shields.io/github/actions/workflow/status/yuma140902/tempura/ci.yml?logo=apple&logoColor=white&label=CI%20on%20macOS)](https://github.com/yuma140902/tempura/actions)
[![](https://img.shields.io/crates/v/tempura?color=blue)](https://crates.io/crates/tempura)
[![](https://img.shields.io/docsrs/tempura)](https://docs.rs/tempura/)

Tempura is a pipeline-based Static Site Generator (SSG) written in Rust. You can define pipelines to generate your site from various resources such as Markdown, JSON, plain text, [Handlebars](https://handlebarsjs.com/) templates, pictures, etc.

## Installation

### Build from source

```sh
cargo install tempura
```

### Download binary

You can download a binary archive from [releases page](https://github.com/yuma140902/tempura/releases).

```sh
# Run one of the following to download binary archive
wget https://github.com/yuma140902/tempura/releases/latest/download/tempura-aarch64-apple-darwin.tar.gz
wget https://github.com/yuma140902/tempura/releases/latest/download/tempura-aarch64-unknown-linux-gnu.tar.gz
wget https://github.com/yuma140902/tempura/releases/latest/download/tempura-aarch64-unknown-linux-musl.tar.gz
wget https://github.com/yuma140902/tempura/releases/latest/download/tempura-i686-pc-windows-msvc.zip
wget https://github.com/yuma140902/tempura/releases/latest/download/tempura-i686-unknown-linux-gnu.tar.gz
wget https://github.com/yuma140902/tempura/releases/latest/download/tempura-i686-unknown-linux-musl.tar.gz
wget https://github.com/yuma140902/tempura/releases/latest/download/tempura-x86_64-apple-darwin.tar.gz
wget https://github.com/yuma140902/tempura/releases/latest/download/tempura-x86_64-pc-windows-msvc.zip
wget https://github.com/yuma140902/tempura/releases/latest/download/tempura-x86_64-unknown-linux-musl.tar.gz 
```

Or you can download and install with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall).

```sh
cargo binstall tempura
```

## Example

### yuma14.net

- Repo: <https://github.com/yuma140902/yuma14net-old/>
- Generated website: <https://old.yuma14.net/>

## Documentation

https://yuma14.net/tempura-doc/

## How to make releases

1. `release/v1.2.3` のような名前のブランチを切る
2. `Cargo.toml` のバージョン番号を更新する
3. 手元で `cargo build` を実行して `Cargo.lock` を更新する
4. Pull Request を作る。[pre-release.yml](.github/workflows/pre-release.yml) が実行されて [CHANGELOG.md](CHANGELOG.md) が更新される。
5. すべてのチェックが通れば Pull Request をクローズする。[release.yml](.github/workflows/release.yml) が実行されて crates.io と GitHub のリリースページにリリースが作られる。
6. ブランチは削除せずに残しておくものとする
