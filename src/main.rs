mod eventloop;
mod view;

use chrono::{DateTime, Local};
use cursive::{
    theme::{BaseColor::*, Color::*, ColorStyle, Palette, PaletteColor::*, Style},
    utils::span::SpannedString,
    view::{Margins, Selector},
    views::{self, TextView},
    Cursive, Vec2, View,
};
use view::build_list_tag;

use crate::view::tag_view::BracketConfig;

#[tokio::main]
async fn main() {
    let mut siv = cursive::default();
    let theme = init_theme();
    siv.set_theme(theme);

    siv.add_fullscreen_layer(views::Layer::new(views::ResizedView::with_full_screen(
        views::LinearLayout::vertical()
            .child(views::PaddedView::new(Margins::tb(0, 1), init_stat()))
            .child(build_body()),
    )));
    let handle = siv.cb_sink().clone();

    tokio::spawn(time_update_loop(handle));

    tokio::task::block_in_place(|| siv.run());
}

async fn time_update_loop(
    handle: crossbeam::channel::Sender<Box<dyn FnOnce(&mut Cursive) + 'static + Send>>,
) -> ! {
    let mut timer = tokio::time::interval(std::time::Duration::from_millis(100));
    let mut time = chrono::Local::now();
    loop {
        timer.tick().await;
        let new_time = chrono::Local::now();
        if new_time.timestamp() != time.timestamp() {
            time = new_time;
        } else {
            continue;
        }
        handle
            .send(Box::new(move |c| {
                c.call_on(&Selector::Name("time"), |s: &mut TextView| {
                    s.set_content(format_current_time(time));
                });
            }))
            .unwrap();
    }
}

fn init_stat() -> impl cursive::View {
    let stat = "NOMINAL";
    let time = format_current_time(Local::now());

    let stat_view = views::NamedView::new(
        "stat",
        views::TextView::new(stat).style(ColorStyle::front(Light(Green))),
    );
    let bar = views::TextView::new("|").style(ColorStyle::front(Light(Black)));
    let time_view = views::NamedView::new("time", views::TextView::new(time));

    views::ResizedView::with_fixed_height(
        1,
        views::LinearLayout::horizontal()
            .child(stat_view)
            .child(views::PaddedView::new(Margins::lr(1, 1), bar))
            .child(time_view),
    )
}

fn format_current_time(time: chrono::DateTime<Local>) -> String {
    let time = time.format("%Y-%m-%d %H:%M:%S");
    time.to_string()
}

fn init_theme() -> cursive::theme::Theme {
    let mut theme = cursive::theme::Theme::default();
    let palette = init_palette();

    theme.borders = cursive::theme::BorderStyle::None;
    theme.palette = palette;

    theme
}

fn init_palette() -> Palette {
    let mut palette = Palette::default();
    palette.extend(vec![
        (Background, TerminalDefault),
        (Shadow, TerminalDefault),
        (View, TerminalDefault),
        (Primary, Dark(White)),
        (Secondary, Light(Black)),
    ]);
    palette
}

fn build_body() -> impl View {
    // let tag = build_list_tag(
    //     255,
    //     vec!["foo", "bar", "baz"].into_iter(),
    //     vec!["qux"].into_iter(),
    //     false,
    //     true,
    // );
    // views::TextView::new(tag)
    view::tag_view::TagView {
        multiline: false,
        counter: 1,
        counter_style: Style::default(),
        tags: vec!["西点子 DD 群".into(), "Rynco Maekawa".into()],
        bracket: BracketConfig {
            left: view::tag_view::BracketStyle::Square,
            right: view::tag_view::BracketStyle::Angle,
        },
        content: "喵喵喵".into(),
        timestamp: DateTime::parse_from_rfc3339("2021-05-15T18:03:02+08:00")
            .unwrap()
            .with_timezone(&Local),
        size: Vec2::new(0, 0),
        dirty: true,
    }
}
