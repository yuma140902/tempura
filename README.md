# Tempura

[![](https://img.shields.io/github/actions/workflow/status/yuma140902/tempura/ci.yml?logo=linux&logoColor=white&label=CI%20on%20Linux)](https://github.com/yuma140902/tempura/actions)
[![](https://img.shields.io/github/actions/workflow/status/yuma140902/tempura/ci.yml?logo=windows&logoColor=white&label=CI%20on%20Windows)](https://github.com/yuma140902/tempura/actions)
[![](https://img.shields.io/github/actions/workflow/status/yuma140902/tempura/ci.yml?logo=apple&logoColor=white&label=CI%20on%20macOS)](https://github.com/yuma140902/tempura/actions)
[![](https://img.shields.io/crates/v/tempura?color=blue)](https://crates.io/crates/tempura)
[![](https://img.shields.io/docsrs/tempura)](https://docs.rs/tempura/)

Tempura is a Static Site Generator (SSG) written in Rust. It can generate HTML from Markdown documents and Handlebars templates. It can also handle static content including images and CSS files, and resolve paths accurately.

## Installation

### Build from source

```sh
cargo install tempura
```

### Download binary

You can download a binary archive from [releases page](https://github.com/yuma140902/tempura/releases).

```sh
# run one of the following to download binary archive
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

### WebTools

- Repo: <https://github.com/yuma140902/webtools>
- Generated website: <https://www.yuma14.net/webtools/>

### tempura-example

- Repo: <https://github.com/yuma140902/tempura-example>
- Generated website: <https://yuma140902.github.io/tempura-example/sample.html>

## Documentation

https://yuma140902.github.io/tempura-doc/

## General Usage

### 1. Setup project

Run `tempura init my_project`.

The following directories and files will be created.

```text
my_project
│  tempura.json
│
├─public
└─src
    ├─pages
    │  │  sample.md
    │  │  style.css
    │  │
    │  └─sub_dir
    │          sample2.md
    │
    └─templates
            page.html.hbs
```

### 2. Edit

Edit template files and markdown files as you like.

See also [Handlebars Language Guide](https://handlebarsjs.com/guide/) and [CommonMark Specification](https://spec.commonmark.org/current/). It is possible to write front matter, which is out of CommonMark specification.

### 3. Build

Run `cd my_project && tempura build`.

HTML files are generated in the `my_project/public/` directory.

### 4. Deploy

Copy contents of `my_project/public/` to your server.
