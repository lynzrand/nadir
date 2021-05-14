mod eventloop;
mod view;

use cursive::{
    theme::{BaseColor::*, Color::*, ColorStyle, Palette, PaletteColor::*},
    view::{Margins, Selector},
    views::{self, TextView},
    View,
};
use view::build_list_tag;

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

    tokio::spawn(async move {
        let mut timer = tokio::time::interval(std::time::Duration::from_millis(250));
        loop {
            timer.tick().await;
            handle
                .send(Box::new(|c| {
                    c.call_on(&Selector::Name("time"), |s: &mut TextView| {
                        s.set_content(format_current_time());
                    });
                }))
                .unwrap();
        }
    });

    tokio::task::block_in_place(|| siv.run());
}

fn init_stat() -> impl cursive::View {
    let stat = "NOMINAL";
    let time = format_current_time();

    let stat_view = views::NamedView::new(
        "stat",
        views::TextView::new(stat).style(ColorStyle::front(Light(Green))),
    );
    let bar = views::TextView::new("|").style(ColorStyle::front(Light(Black)));
    let time_view = views::NamedView::new("time", views::TextView::new(time));

    views::LinearLayout::horizontal()
        .child(stat_view)
        .child(views::PaddedView::new(Margins::lr(1, 1), bar))
        .child(time_view)
}

fn format_current_time() -> String {
    let time = chrono::Local::now();
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
    let tag = build_list_tag(
        255,
        vec!["foo", "bar", "baz"].into_iter(),
        vec!["qux"].into_iter(),
        false,
        true,
    );
    views::TextView::new(tag)
}
