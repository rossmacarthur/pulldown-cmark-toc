//! Generate a table of contents from a Markdown document.
//!
//! By default the heading anchor calculation (aka the "slugification")
//! is done in a way that attempts to mimic GitHub's (undocumented) behavior.
//! (Though you can customize this with your own [`slug::Slugify`] implementation).
//!
//! # Examples
//!
//! ```
//! use pulldown_cmark_toc::TableOfContents;
//!
//! let text = "# Heading\n\n## Subheading\n\n## Subheading with `code`\n";
//!
//! let toc = TableOfContents::new(text);
//! assert_eq!(
//!     toc.to_cmark(),
//!     r#"- [Heading](#heading)
//!   - [Subheading](#subheading)
//!   - [Subheading with `code`](#subheading-with-code)
//! "#
//! );
//! ```

mod render;
mod slug;

use std::borrow::Borrow;
use std::fmt::Write;
use std::slice::Iter;

pub use pulldown_cmark::HeadingLevel;
use pulldown_cmark::{Event, Options as CmarkOptions, Parser, Tag, TagEnd};

pub use render::{ItemSymbol, Options};
pub use slug::{GitHubSlugifier, Slugify};

/////////////////////////////////////////////////////////////////////////
// Definitions
/////////////////////////////////////////////////////////////////////////

/// Represents a heading.
#[derive(Debug, Clone)]
pub struct Heading<'a> {
    /// The Markdown events between the heading tags.
    events: Vec<Event<'a>>,
    /// The heading level.
    level: HeadingLevel,
}

/// Represents a Table of Contents.
#[derive(Debug)]
pub struct TableOfContents<'a> {
    headings: Vec<Heading<'a>>,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

impl Heading<'_> {
    /// The raw events contained between the heading tags.
    pub fn events(&self) -> Iter<Event> {
        self.events.iter()
    }

    /// The heading level.
    pub fn level(&self) -> HeadingLevel {
        self.level
    }

    /// The heading text with all Markdown code stripped out.
    ///
    /// The output of this this function can be used to generate an anchor.
    pub fn text(&self) -> String {
        let mut buf = String::new();
        for event in self.events() {
            if let Event::Text(s) | Event::Code(s) = event {
                buf.push_str(s);
            }
        }
        buf
    }
}

impl<'a> TableOfContents<'a> {
    /// Construct a new table of contents from Markdown text.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pulldown_cmark_toc::TableOfContents;
    /// let toc = TableOfContents::new("# Heading\n");
    /// ```
    pub fn new(text: &'a str) -> Self {
        // We are not enabling all options since we want to mimic
        // GitHub's behavior as closely as possible by default.
        // And e.g. enabling heading attributes could result in wrong anchors
        // or enabling smart punctuation would result in inconsistent rendering.
        let mut options = CmarkOptions::empty();
        options.insert(CmarkOptions::ENABLE_STRIKETHROUGH);
        options.insert(CmarkOptions::ENABLE_FOOTNOTES);
        // Not enabling tables and tasklists since they cannot have any
        // effect on headings (which are the only events we care about).
        let events = Parser::new_ext(text, options);
        Self::new_with_events(events)
    }

    /// Construct a new table of contents from parsed Markdown events.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pulldown_cmark_toc::TableOfContents;
    /// use pulldown_cmark::Parser;
    ///
    /// let parser = Parser::new("# Heading\n");
    /// let toc = TableOfContents::new_with_events(parser);;
    /// ```
    pub fn new_with_events<I, E>(events: I) -> Self
    where
        I: Iterator<Item = E>,
        E: Borrow<Event<'a>>,
    {
        let mut headings = Vec::new();
        let mut current: Option<Heading> = None;

        for event in events {
            let event = event.borrow();
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    current = Some(Heading {
                        events: Vec::new(),
                        level: *level,
                    });
                }
                Event::End(TagEnd::Heading(level)) => {
                    let heading = current.take().unwrap();
                    assert_eq!(heading.level, *level);
                    headings.push(heading);
                }
                event => {
                    if let Some(heading) = current.as_mut() {
                        heading.events.push(event.clone());
                    }
                }
            }
        }
        Self { headings }
    }

    /// Iterate over the headings in this table of contents.
    ///
    /// # Examples
    ///
    /// Simple iteration over each heading.
    /// ```
    /// # use pulldown_cmark_toc::TableOfContents;
    /// let toc = TableOfContents::new("# Heading\n");
    ///
    /// for heading in toc.headings() {
    ///     // use heading
    /// }
    /// ```
    ///
    /// Filtering out certain heading levels.
    /// ```
    /// # use pulldown_cmark_toc::{HeadingLevel, TableOfContents};
    /// let toc = TableOfContents::new("# Heading\n## Subheading\n");
    ///
    /// for heading in toc.headings().filter(|h| h.level() >= HeadingLevel::H2) {
    ///     // use heading
    /// }
    /// ```
    pub fn headings(&self) -> Iter<Heading> {
        self.headings.iter()
    }

    /// Render the table of contents as Markdown.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pulldown_cmark_toc::TableOfContents;
    /// let toc = TableOfContents::new("# Heading\n## Subheading\n");
    /// assert_eq!(
    ///     toc.to_cmark(),
    ///     "- [Heading](#heading)\n  - [Subheading](#subheading)\n"
    /// );
    /// ```
    #[must_use]
    pub fn to_cmark(&self) -> String {
        self.to_cmark_with_options(Options::default())
    }

    /// Render the table of contents as Markdown with extra options.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pulldown_cmark_toc::{HeadingLevel, ItemSymbol, Options, TableOfContents};
    ///
    /// let toc = TableOfContents::new("# Heading\n## Subheading\n");
    /// let options = Options::default()
    ///     .item_symbol(ItemSymbol::Asterisk)
    ///     .levels(HeadingLevel::H2..=HeadingLevel::H6)
    ///     .indent(4);
    /// assert_eq!(
    ///     toc.to_cmark_with_options(options),
    ///     "* [Subheading](#subheading)\n"
    /// );
    /// ```
    #[must_use]
    pub fn to_cmark_with_options(&self, options: Options) -> String {
        let Options {
            item_symbol,
            levels,
            indent,
            slugifier: mut slugger,
        } = options;

        let mut buf = String::new();
        for heading in self.headings().filter(|h| levels.contains(&h.level())) {
            let title = crate::render::to_cmark(heading.events());
            let indent = indent * (heading.level() as usize - *levels.start() as usize);

            // make sure the anchor is unique

            writeln!(
                buf,
                "{:indent$}{} [{}](#{})",
                "",
                item_symbol,
                title,
                slugger.slugify(&heading.text()),
                indent = indent,
            )
            .unwrap();
        }
        buf
    }
}

/////////////////////////////////////////////////////////////////////////
// Unit tests
/////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    use pulldown_cmark::CowStr::Borrowed;
    use pulldown_cmark::Event::{Code, Text};

    #[test]
    fn heading_text_with_code() {
        let heading = Heading {
            events: vec![Code(Borrowed("Another")), Text(Borrowed(" heading"))],
            level: HeadingLevel::H1,
        };
        assert_eq!(heading.text(), "Another heading");
    }

    #[test]
    fn heading_text_with_links() {
        let events = Parser::new("Here [TOML](https://toml.io)").collect();
        let heading = Heading {
            events,
            level: HeadingLevel::H1,
        };
        assert_eq!(heading.text(), "Here TOML");
    }

    #[test]
    fn toc_new() {
        let toc = TableOfContents::new("# Heading\n\n## `Another` heading\n");
        assert_eq!(toc.headings[0].events, [Text(Borrowed("Heading"))]);
        assert_eq!(toc.headings[0].level, HeadingLevel::H1);
        assert_eq!(
            toc.headings[1].events,
            [Code(Borrowed("Another")), Text(Borrowed(" heading"))]
        );
        assert_eq!(toc.headings[1].level, HeadingLevel::H2);
        assert_eq!(toc.headings.len(), 2);
    }

    #[test]
    fn toc_new_does_not_enable_smart_punctuation() {
        let toc = TableOfContents::new("# What's the deal with ellipsis ...?\n");
        assert_eq!(toc.headings[0].text(), "What's the deal with ellipsis ...?");
    }

    #[test]
    fn toc_new_does_not_enable_heading_attributes() {
        let toc = TableOfContents::new("# text { #id .class1 .class2 }\n");
        assert_eq!(toc.headings[0].text(), "text { #id .class1 .class2 }");
    }

    #[test]
    fn toc_to_cmark_unique_anchors() {
        let toc = TableOfContents::new("# Heading\n\n# Heading\n\n# `Heading`");
        assert_eq!(
            toc.to_cmark(),
            "- [Heading](#heading)\n- [Heading](#heading-1)\n- [`Heading`](#heading-2)\n"
        )
    }
}
