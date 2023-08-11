use std::{borrow::Cow, collections::HashMap};

use once_cell::sync::Lazy;
use regex::Regex;

/// A trait to specify the anchor calculation.
pub trait Slugify {
    fn slugify<'a>(&mut self, str: &'a str) -> Cow<'a, str>;
}

/// A slugifier that attempts to mimic GitHub's behavior.
///
/// Unfortunately GitHub's behavior is not documented anywhere by GitHub.
/// This should really be part of the [GitHub Flavored Markdown Spec][gfm]
/// but alas it's not. And there also does not appear to be a public issue
/// tracker for the spec where that issue could be raised.
///
/// [gfm]: https://github.github.com/gfm/
#[derive(Default)]
pub struct GitHubSlugifier {
    counts: HashMap<String, i32>,
}

impl Slugify for GitHubSlugifier {
    fn slugify<'a>(&mut self, str: &'a str) -> Cow<'a, str> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\w\- ]").unwrap());
        let anchor = RE
            .replace_all(&str.to_lowercase().replace(' ', "-"), "")
            .into_owned();

        let i = self
            .counts
            .entry(anchor.clone())
            .and_modify(|i| *i += 1)
            .or_insert(0);

        match *i {
            0 => anchor,
            i => format!("{}-{}", anchor, i),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::slug::{GitHubSlugifier, Slugify};
    use crate::Heading;
    use pulldown_cmark::CowStr::Borrowed;
    use pulldown_cmark::Event::{Code, Text};
    use pulldown_cmark::{HeadingLevel, Parser};

    #[test]
    fn heading_anchor_with_code() {
        let heading = Heading {
            events: vec![Code(Borrowed("Another")), Text(Borrowed(" heading"))],
            level: HeadingLevel::H1,
        };
        assert_eq!(
            GitHubSlugifier::default().slugify(&heading.text()),
            "another-heading"
        );
    }

    #[test]
    fn heading_anchor_with_links() {
        let events = Parser::new("Here [TOML](https://toml.io)").collect();
        let heading = Heading {
            events,
            level: HeadingLevel::H1,
        };
        assert_eq!(
            GitHubSlugifier::default().slugify(&heading.text()),
            "here-toml"
        );
    }

    #[test]
    fn github_slugger_non_ascii_lowercase() {
        assert_eq!(GitHubSlugifier::default().slugify("Привет"), "привет");
    }
}
