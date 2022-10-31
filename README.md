# Tempura

Static site generator (SSG) using handlebars and pulldown-cmark, written in Rust.


## Installation

```sh
cargo install tempura
```

## Example

- Repo: <https://github.com/yuma140902/tempura-example>
- Generated website: <https://yuma140902.github.io/tempura-example/sample.html>

## Usage

### 1. Setup project

Run `tempura init my_project`.

The following directories and files will be created.

```
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

See also [Handlebars Lanugage Guide](https://handlebarsjs.com/guide/) and [CommonMark Specification](https://spec.commonmark.org/current/). It is possible to write front matter, which is out of CommonMark specification.

### 3. Build

Run `cd my_project && tempura build`.

HTML files are generated in `my_project/public/` directory.

### 4. Deploy

Copy contents of `my_project/public/` to your server.
