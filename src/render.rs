//! Render a limited set of Markdown events back to Markdown.

use std::borrow::Borrow;
use std::fmt;
use std::fmt::Write;
use std::ops::RangeInclusive;

use pulldown_cmark::{Event, HeadingLevel, Tag};

use crate::slug::{GitHubSlugifier, Slugify};

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
/// # use pulldown_cmark_toc::{HeadingLevel, ItemSymbol, Options};
///
/// let options = Options::default()
///     .item_symbol(ItemSymbol::Asterisk)
///     .levels(HeadingLevel::H2..=HeadingLevel::H6)
///     .indent(4);
///
/// ```
pub struct Options {
    pub(crate) item_symbol: ItemSymbol,
    pub(crate) levels: RangeInclusive<HeadingLevel>,
    pub(crate) indent: usize,
    pub(crate) slugifier: Box<dyn Slugify>,
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
            levels: (HeadingLevel::H1..=HeadingLevel::H6),
            indent: 2,
            slugifier: Box::new(GitHubSlugifier::default()),
        }
    }
}

impl Options {
    /// The symbol to use for Table of Contents list items.
    #[must_use]
    pub fn item_symbol(mut self, item_symbol: ItemSymbol) -> Self {
        self.item_symbol = item_symbol;
        self
    }

    /// Only levels in the given range will be rendered.
    #[must_use]
    pub fn levels(mut self, levels: RangeInclusive<HeadingLevel>) -> Self {
        self.levels = levels;
        self
    }

    /// The number of spaces to use for indentation between heading levels.
    #[must_use]
    pub fn indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }

    /// The slugifier to use for the heading anchors.
    #[must_use]
    pub fn slugifier(mut self, slugifier: Box<dyn Slugify>) -> Self {
        self.slugifier = slugifier;
        self
    }
}
