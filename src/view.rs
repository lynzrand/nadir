pub mod tag_view;
pub mod tagged_group;

use std::fmt::{Display, Write};

use cursive::{view::Position, Vec2, View};

pub fn build_list_tag<'a>(
    count: u64,
    namespace: impl Iterator<Item = &'a str> + Clone,
    tags: impl Iterator<Item = &'a str> + Clone,
    bra: bool,
    ket: bool,
) -> String {
    let mut s = String::new();

    // format count section: "[NNN" or "|" or "<"
    if count > 1 {
        s.write_fmt(format_args!("[{}", SimplifyNumber(count)))
            .unwrap();
    } else if bra {
        s.push('[');
    } else {
        s.push('|');
    }

    // format namespace and tags section
    s.write_fmt(format_args!(
        "{}",
        Separated('|', '|', if ket { '>' } else { ']' }, namespace.chain(tags))
    ))
    .unwrap();

    s
}

/// Format a number to at most 4 chars
struct SimplifyNumber(u64);

impl Display for SimplifyNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 < 10000 {
            write!(f, "{}", self.0)
        } else if self.0 < 1_000_000 {
            write!(f, "{}k", self.0 / 1000)
        } else if self.0 < 1_000_000_000 {
            write!(f, "{}M", self.0 / 1_000_000)
        } else if self.0 < 1_000_000_000_000 {
            write!(f, "{}T", self.0 / 1_000_000_000)
        } else {
            write!(f, "{}E", self.0 / 1_000_000_000_000)
        }
    }
}

struct Separated<I>(char, char, char, I);

impl<'a, I, S> Display for Separated<I>
where
    I: Iterator<Item = S> + Clone,
    S: Into<&'a str>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.3.clone();
        let first = it.next();
        if first.is_none() {
            return Ok(());
        }
        let first = first.unwrap();
        write!(f, "{}{}", self.0, first.into())?;
        for rest in it {
            write!(f, "{}{}", self.1, rest.into())?;
        }
        write!(f, "{}", self.2)
    }
}
