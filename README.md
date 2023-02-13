# Tempura

[![](https://badgen.net/github/checks/yuma140902/tempura/master/ubuntu?label=linux)](https://github.com/yuma140902/tempura/actions/workflows/ci.yml)
[![](https://badgen.net/github/checks/yuma140902/tempura/master/windows?icon=windows)](https://github.com/yuma140902/tempura/actions/workflows/ci.yml)
[![](https://badgen.net/github/checks/yuma140902/tempura/master/macos?icon=apple)](https://github.com/yuma140902/tempura/actions/workflows/ci.yml)
[![](https://badgen.net/crates/v/tempura?color=blue)](https://crates.io/crates/tempura)
[![](https://docs.rs/tempura/badge.svg)](https://docs.rs/tempura/)

Tempura is a Static Site Generator (SSG) written in Rust. It can generate HTML from Markdown documents and Handlebars templates. It can also handle static content including images and CSS files, and resolve paths accurately.

## Installation

### Build from source

```sh
cargo install tempura
```

### Install binary

You can download a binary from [releases page](https://github.com/yuma140902/tempura/releases).

Or you can download with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall).

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
