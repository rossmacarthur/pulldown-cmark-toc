# pulldown-cmark-toc

[![Crates.io version](https://img.shields.io/crates/v/pulldown-cmark-toc.svg)](https://crates.io/crates/pulldown-cmark-toc)
[![Build status](https://img.shields.io/github/workflow/status/rossmacarthur/pulldown-cmark-toc/build/trunk)](https://github.com/rossmacarthur/pulldown-cmark-toc/actions?query=workflow%3Abuild)

Generate a table of contents from a Markdown document.

## Getting started

Add the following dependency to your `Cargo.toml`.

```toml
[dependencies]
pulldown-cmark-toc = "0.1"
```

## Usage

```rust
use pulldown_cmark_toc::TableOfContents;

let text = r#"
# Heading

## Subheading

## Subheading with `code`
"#;

let toc = TableOfContents::new(text);
assert_eq!(
    toc.to_cmark(),
    r#"- [Heading](#heading)
  - [Subheading](#subheading)
  - [Subheading with `code`](#subheading-with-code)
"#
);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
