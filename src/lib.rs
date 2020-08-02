use std::borrow::Borrow;
use std::slice::Iter;

use lazy_static::lazy_static;
use pulldown_cmark::{Event, Options, Parser, Tag};
use regex::Regex;

/////////////////////////////////////////////////////////////////////////
// Definitions
/////////////////////////////////////////////////////////////////////////

/// Represents a heading.
#[derive(Debug, Clone)]
pub struct Heading<'a> {
    /// The Markdown events between the heading tags.
    events: Vec<Event<'a>>,
    /// The heading level.
    level: u32,
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
    pub fn level(&self) -> u32 {
        self.level
    }

    /// The heading text with all Markdown code stripped out.
    ///
    /// The output of this this function can used to generate an anchor.
    pub fn text(&self) -> String {
        let mut buf = String::new();
        for event in self.events() {
            if let Event::Text(s) | Event::Code(s) = event {
                buf.push_str(&s);
            }
        }
        buf
    }

    /// The generated anchor link.
    ///
    /// This is calculated in the same way that GitHub calculates it.
    pub fn anchor(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[^\w\- ]").unwrap();
        }
        RE.replace_all(&self.text().to_ascii_lowercase().replace(" ", "-"), "")
            .into_owned()
    }
}

impl<'a> TableOfContents<'a> {
    /// Construct a new table of contents from Markdown text.
    pub fn from_str(text: &'a str) -> Self {
        let events = Parser::new_ext(text, Options::all());
        Self::from_events(events)
    }

    /// Construct a new table of contents from parsed Markdown events.
    pub fn from_events<I, E>(events: I) -> Self
    where
        I: Iterator<Item = E>,
        E: Borrow<Event<'a>>,
    {
        let mut current: Option<Heading> = None;
        let mut headings = Vec::new();

        for event in events {
            let event = event.borrow();
            match &*event {
                Event::Start(Tag::Heading(level)) => {
                    current = Some(Heading {
                        events: Vec::new(),
                        level: *level,
                    });
                }
                Event::End(Tag::Heading(level)) => {
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
    pub fn headings(&self) -> Iter<Heading> {
        self.headings.iter()
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
            level: 1,
        };
        assert_eq!(heading.text(), "Another heading");
    }

    #[test]
    fn heading_text_with_links() {
        let events = Parser::new("Here [TOML](https://toml.io)").collect();
        let heading = Heading { events, level: 1 };
        assert_eq!(heading.text(), "Here TOML");
    }

    #[test]
    fn heading_anchor_with_code() {
        let heading = Heading {
            events: vec![Code(Borrowed("Another")), Text(Borrowed(" heading"))],
            level: 1,
        };
        assert_eq!(heading.anchor(), "another-heading");
    }

    #[test]
    fn heading_anchor_with_links() {
        let events = Parser::new("Here [TOML](https://toml.io)").collect();
        let heading = Heading { events, level: 1 };
        assert_eq!(heading.anchor(), "here-toml");
    }

    #[test]
    fn toc_from_str() {
        let toc = TableOfContents::from_str("# Heading\n\n## `Another` heading\n");
        assert_eq!(toc.headings[0].events, [Text(Borrowed("Heading"))]);
        assert_eq!(toc.headings[0].level, 1);
        assert_eq!(
            toc.headings[1].events,
            [Code(Borrowed("Another")), Text(Borrowed(" heading"))]
        );
        assert_eq!(toc.headings[1].level, 2);
        assert_eq!(toc.headings.len(), 2);
    }
}
