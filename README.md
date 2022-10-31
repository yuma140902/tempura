# Tempura

Static site generator (SSG) using handlebars and pulldown-cmark, written in Rust.


## Installation

```sh
cargo install tempura
```

## Usage

### 1. Run `tempura init sample`

The following directories and files will be created.

```
sample
├─pages
│  │  sample.md
│  └─sub_dir
│          sample2.md
├─public
└─templates
        page.html.hbs
```

### 2. Edit template files and markdown files as you like

See also [Handlebars Lanugage Guide](https://handlebarsjs.com/guide/) and [CommonMark Specification](https://spec.commonmark.org/current/). It is possible to write front matter, which is out of CommonMark specification.

### 3. Run `tempura gen sample`

HTML files are generated in `sample/public/` directory.

### 4. Copy contents of `sample/public/` to your server.