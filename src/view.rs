pub mod group_list_view;
pub mod group_view;
pub mod tag_view;

use std::fmt::{Display, Write};

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
