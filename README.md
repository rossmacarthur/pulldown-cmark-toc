# pulldown-cmark-toc

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

let toc = TableOfContents::from_str(text);

for heading in toc.headings() {
    let indent = (2 * (heading.level() - 1)) as usize;
    println!(
        "{:indent$}* [{}]({})",
        "",
        heading.text(),
        heading.anchor(),
        indent = indent
    );
}
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
