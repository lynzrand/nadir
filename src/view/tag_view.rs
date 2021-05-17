use chrono::{DateTime, Datelike, Duration, Local};
use cursive::{
    event::EventResult,
    theme::{ColorStyle, Effect, Style},
    utils::{markup::StyledString, span::SpannedText},
    Rect, Vec2, View,
};
use unicode_truncate::UnicodeTruncateStr;
use unicode_width::*;

use crate::view::SimplifyNumber;

/// Style of bracket
#[derive(Debug, Clone, Copy)]
pub enum BracketStyle {
    /// '|'
    Line,
    /// '[' '];
    Square,
    /// '<' '>'
    Angle,
    /// '(' ')'
    Round,
}

impl BracketStyle {
    pub fn left(&self) -> char {
        match self {
            BracketStyle::Line => '|',
            BracketStyle::Square => '[',
            BracketStyle::Angle => '<',
            BracketStyle::Round => '(',
        }
    }

    pub fn left_str(&self) -> &'static str {
        match self {
            BracketStyle::Line => "|",
            BracketStyle::Square => "[",
            BracketStyle::Angle => "<",
            BracketStyle::Round => "(",
        }
    }

    pub fn right(&self) -> char {
        match self {
            BracketStyle::Line => '|',
            BracketStyle::Square => ']',
            BracketStyle::Angle => '>',
            BracketStyle::Round => ')',
        }
    }

    pub fn right_str(&self) -> &'static str {
        match self {
            BracketStyle::Line => "|",
            BracketStyle::Square => "]",
            BracketStyle::Angle => ">",
            BracketStyle::Round => ")",
        }
    }
}

/// A specific bracket style configuration.
pub struct BracketConfig {
    pub left: BracketStyle,
    pub right: BracketStyle,
}

pub struct TagView {
    pub multiline: bool,
    pub counter: u64,
    pub counter_style: Style,
    pub tags: Vec<StyledString>,
    pub bracket: BracketConfig,
    pub content: StyledString,
    pub timestamp: DateTime<Local>,

    // ----
    layout: TagViewLayout,
    dirty: bool,
}

#[derive(Default)]
struct TagViewLayout {
    pub size: Vec2,
    pub widths: Vec<usize>,
    pub truncate_tags: Option<usize>,
}

impl TagView {
    pub fn new(
        multiline: bool,
        counter: u64,
        counter_style: Style,
        tags: Vec<StyledString>,
        bracket: BracketConfig,
        content: StyledString,
        timestamp: DateTime<Local>,
    ) -> Self {
        Self {
            multiline,
            counter,
            counter_style,
            tags,
            bracket,
            content,
            timestamp,
            layout: TagViewLayout::default(),
            dirty: true,
        }
    }

    fn print_counter(&self) -> bool {
        self.counter > 1
    }

    fn print_tags(&self) -> bool {
        !self.tags.is_empty()
    }

    /// Do layout and truncate tags when they are too long
    fn calc_truncation(&mut self, _size: Vec2) {
        self.layout.widths.clear();
        self.layout
            .widths
            .extend(self.tags.iter().map(|x| x.width()));
        let widths = &self.layout.widths;
        let total_size = widths.iter().copied().sum::<usize>();
        let total_size_with_sep = total_size + self.tags.len() - 1;
        let mut truncate = None;
        let max_size = self.layout.size.x / 2;

        if total_size_with_sep > max_size {
            const MIN_LEN_TAGS: usize = 4;

            // The text is too long, we need to truncate some tags.
            // We try to truncate all tags below a certain length.
            {
                let mut sizes_sorted = widths.clone();
                sizes_sorted.sort_unstable_by(|a, b| a.cmp(b).reverse());
                let mut truncate_cnt = 0;
                let mut truncate_to = sizes_sorted[0];
                let mut total_width = total_size_with_sep;
                let truncate_to = loop {
                    truncate_cnt += 1;
                    while sizes_sorted.get(truncate_cnt).copied() == Some(truncate_to) {
                        truncate_cnt += 1;
                    }
                    let new_truncate_to = sizes_sorted.get(truncate_cnt).copied().unwrap_or(0);

                    let new_total_width =
                        total_width - (truncate_to - new_truncate_to) * truncate_cnt;
                    if new_total_width <= max_size {
                        break new_truncate_to + (max_size - new_total_width) / truncate_cnt;
                    } else {
                        truncate_to = new_truncate_to;
                        total_width = new_total_width;
                    }
                };
                truncate = Some(std::cmp::max(truncate_to, MIN_LEN_TAGS));
            }
        }
        self.layout.truncate_tags = truncate;
    }

    /// Print the counter part
    fn do_print_counter(&self, start: Vec2, printer: &cursive::Printer, style: Style) -> Vec2 {
        let cur = start;
        let counter = format!("{}", SimplifyNumber(self.counter));
        let width = counter.width();
        printer.with_style(style, |p| p.print(cur, &counter));
        cur.map_x(|x| x + width)
    }

    fn do_print_tags(
        &self,
        start: Vec2,
        max_size: usize,
        printer: &cursive::Printer,
        secondary_style: ColorStyle,
    ) -> Vec2 {
        // early return when there's nothing to print
        if self.tags.is_empty() {
            return start;
        }
        let truncate = self.layout.truncate_tags;
        let widths = &self.layout.widths;

        let mut cur = start;
        let mut first = true;

        // print the tags
        for (s, &width) in self.tags.iter().zip(widths) {
            if first {
                first = false;
            } else {
                printer.with_style(secondary_style, |p| p.print(cur, "|"));
                cur.x += 1;
            }

            let width = if truncate.map(|x| x < width).unwrap_or_default() {
                let (_, width) = s.source().unicode_truncate(truncate.unwrap());
                width
            } else {
                width
            };

            if cur.x + width > max_size {
                break;
            }

            let printer = printer.windowed(Rect::from_size(cur, (width, 1)));
            printer.print_styled((0, 0), s.into());
            cur.x += width;
        }
        cur
    }

    fn do_print_content(&self, start: Vec2, printer: &cursive::Printer, width: usize) -> Vec2 {
        let printer = printer.windowed(Rect::from_size(start, (width, 1)));
        printer.print_styled((0, 0), (&self.content).into());
        let content_width = self.content.width();
        if width > content_width {
            printer.print_hline((content_width, 0), width - content_width, " ");
        }
        start.map_x(|x| x + width)
    }

    fn do_print_time(&self, start: Vec2, printer: &cursive::Printer) -> Vec2 {
        let now = chrono::Local::now();
        let duration = now - self.timestamp;
        printer.with_style(ColorStyle::secondary(), |p| {
            if duration < Duration::zero() {
                p.print(start, "now");
            } else if duration < Duration::days(1) {
                p.print(start, &self.timestamp.format("%H:%M").to_string());
            } else if duration < Duration::days(32) {
                p.print(start, &format!("{}d", duration.num_days()));
            } else if duration < Duration::days(366) {
                p.print(
                    start,
                    &format!(
                        "{}d",
                        (now.month0() - self.timestamp.month0() + 12) % 12 + 1
                    ),
                );
            } else {
                p.print(start, &format!("{}y", now.year() - self.timestamp.year()));
            }
        });
        start.map_x(|x| x + 5)
    }
    const TIME_SECTION_SIZE: usize = 6;
}

impl View for TagView {
    fn draw(&self, printer: &cursive::Printer) {
        let tag_size = self.layout.size.x / 2;
        let mut cur_print = Vec2::new(0, 0);

        let bra = if self.print_counter() {
            BracketStyle::Square
        } else {
            self.bracket.left
        };

        let ket = if self.print_counter() && !self.print_tags() {
            BracketStyle::Square
        } else {
            self.bracket.right
        };

        let (secondary_style, base_effect) = if printer.focused {
            (ColorStyle::default(), Effect::Reverse)
        } else {
            (ColorStyle::secondary(), Effect::Simple)
        };

        printer.with_effect(base_effect, |printer| {
            if self.print_counter() || self.print_tags() {
                printer.with_color(secondary_style, |p| {
                    p.print(cur_print, bra.left_str());
                    cur_print.x += 1;
                });

                if self.print_counter() {
                    cur_print = self.do_print_counter(cur_print, printer, self.counter_style);

                    if self.print_tags() {
                        printer.with_color(secondary_style, |p| {
                            p.print(cur_print, "|");
                            cur_print.x += 1;
                        });
                    }
                }
                if self.print_tags() {
                    cur_print = self.do_print_tags(cur_print, tag_size, printer, secondary_style);
                }

                printer.with_color(secondary_style, |p| {
                    p.print(cur_print, ket.right_str());
                    cur_print.x += 1;
                });
            }

            printer.print(cur_print, " ");
            cur_print.x += 1;

            cur_print = self.do_print_content(
                cur_print,
                printer,
                self.layout.size.x - cur_print.x - Self::TIME_SECTION_SIZE,
            );
        });
        cur_print.x += 1;
        self.do_print_time(cur_print, printer);
    }

    fn layout(&mut self, size: Vec2) {
        if size != self.layout.size {
            self.dirty = true;
        }
        self.layout.size = size;
        self.calc_truncation(size);
    }

    fn needs_relayout(&self) -> bool {
        self.dirty
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        if self.multiline {
            todo!("Multiline is not supported")
        } else {
            Vec2::new(constraint.x, 1)
        }
    }

    fn take_focus(&mut self, _source: cursive::direction::Direction) -> bool {
        true
    }

    fn on_event(&mut self, ev: cursive::event::Event) -> EventResult {
        match ev {
            cursive::event::Event::Mouse {
                offset: _,
                position: _,
                event: _,
            } => {
                //
                EventResult::Consumed(None)
            }
            _ => EventResult::Ignored,
        }
    }
}
