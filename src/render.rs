//! Render a limited set of Markdown events back to Markdown.

use std::borrow::Borrow;
use std::fmt::{self, Write};
use std::ops::RangeInclusive;

use pulldown_cmark::{Event, Tag};

/// Which symbol to use when rendering Markdown list items.
pub enum ItemSymbol {
    /// `-`
    Hyphen,
    /// `*`
    Asterisk,
}

/// Configuration options to use when rendering the Table of Contents.
///
/// # Examples
///
/// ```
/// # use pulldown_cmark_toc::{ItemSymbol, Options};
/// let options = Options::default()
///     .item_symbol(ItemSymbol::Asterisk)
///     .levels(2..=6)
///     .indent(4);
///
/// ```
pub struct Options {
    pub(crate) item_symbol: ItemSymbol,
    pub(crate) levels: RangeInclusive<u32>,
    pub(crate) indent: usize,
}

pub(crate) fn to_cmark<'a, I, E>(events: I) -> String
where
    I: Iterator<Item = E>,
    E: Borrow<Event<'a>>,
{
    let mut buf = String::new();
    for event in events {
        let event = event.borrow();
        match event {
            Event::Start(Tag::Emphasis) | Event::End(Tag::Emphasis) => buf.push('*'),
            Event::Start(Tag::Strong) | Event::End(Tag::Strong) => buf.push_str("**"),
            Event::Text(s) => buf.push_str(s),
            Event::Code(s) => {
                buf.push('`');
                buf.push_str(s);
                buf.push('`');
            }
            Event::Html(s) => buf.push_str(s),
            _ => {} // not yet implemented!
        }
    }
    buf
}

impl fmt::Display for ItemSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hyphen => f.write_char('-'),
            Self::Asterisk => f.write_char('*'),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            item_symbol: ItemSymbol::Hyphen,
            levels: (1..=6),
            indent: 2,
        }
    }
}

impl Options {
    /// The symbol to use for Table of Contents list items.
    pub fn item_symbol(mut self, item_symbol: ItemSymbol) -> Self {
        self.item_symbol = item_symbol;
        self
    }

    /// Only levels in the given range will be rendered.
    pub fn levels(mut self, levels: RangeInclusive<u32>) -> Self {
        self.levels = levels;
        self
    }

    /// The number of spaces to use for indentation between heading levels.
    pub fn indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }
}
