# pulldown-cmark-toc

[![Crates.io Version](https://badgers.space/crates/version/pulldown-cmark-toc)](https://crates.io/crates/pulldown-cmark-toc)
[![Docs.rs Latest](https://badgers.space/badge/docs.rs/latest/blue)](https://docs.rs/pulldown-cmark-toc)
[![Build Status](https://badgers.space/github/checks/rossmacarthur/pulldown-cmark-toc/trunk?label=build)](https://github.com/rossmacarthur/pulldown-cmark-toc/actions/workflows/build.yaml?branch=trunk)

Generate a table of contents from a Markdown document.

## Getting started

Add the `pulldown-cmark-toc` to your `Cargo.toml`.

```
cargo add pulldown-cmark-toc
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
