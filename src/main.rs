mod model;
mod opt;
mod server;
mod ui;
mod util;
mod view;

use std::sync::Arc;

use chrono::{DateTime, Local};
use cursive::{
    event::{EventResult, EventTrigger, Key},
    theme::{BaseColor::*, Color::*, ColorStyle, Palette, PaletteColor::*, Style},
    utils::span::SpannedString,
    view::{Margins, Selector, SizeConstraint},
    views::{self, TextView},
    Cursive, Vec2, View,
};
use model::MessageGroup;
use util::DirtyCheckLock;
use view::tagged_group::{GroupRef, GroupView};

use crate::view::tag_view::BracketConfig;

pub type CursiveHandle = crossbeam::channel::Sender<Box<dyn FnOnce(&mut Cursive) + 'static + Send>>;

#[tokio::main]
async fn main() {
    let mut siv = cursive::default();
    let theme = init_theme();
    siv.set_theme(theme);
    // No auto refreshing, use the handle to trigger updates
    // siv.set_fps(1);

    let data = Arc::new(DirtyCheckLock::new(MessageGroup::new(
        nadir_types::model::MessageGroup {
            id: "tg".into(),
            title: "Telegram".into(),
            capacity: 10,
            pinned_capacity: 3,
            importance: 0,
        },
    )));

    siv.add_fullscreen_layer(views::Layer::new(views::ResizedView::with_full_screen(
        views::LinearLayout::vertical()
            .child(views::PaddedView::new(Margins::tb(0, 1), init_stat()))
            .child(build_body(data.clone())),
    )));
    let handle = siv.cb_sink().clone();

    tokio::spawn(time_update_loop(handle.clone()));
    tokio::spawn(data_update_loop(handle, data));

    let crossterm_backend = cursive::backends::crossterm::Backend::init().unwrap();
    let buffered_backend = Box::new(cursive_buffered_backend::BufferedBackend::new(
        crossterm_backend,
    ));

    tokio::task::block_in_place(|| siv.run_with(|| buffered_backend));
}

/// Testing function for updateing data
async fn data_update_loop(handle: CursiveHandle, data: GroupRef) {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    // do some test writes
    {
        let mut guard = data.write();
        let mut meta = guard.meta().clone();
        meta.title = "bar".into();
        guard.set_meta(meta).unwrap();
        handle.send(Box::new(|_c| {})).unwrap();
    }

    let mut rand = 123;
    for i in 0..50 {
        tokio::time::sleep(std::time::Duration::from_millis(rand % 2000)).await;
        rand = rand.wrapping_add(rand << 13).wrapping_add(rand >> 17);

        let mut guard = data.write();
        guard.add_message(nadir_types::model::Message {
            id: format!("FOO{}", rand % 7).into(),
            counter: None,
            tags: vec![format!("foo{}", rand % 7), "bar".into(), format!("#{}", i)],
            body: "喵喵喵喵喵喵".into(),
            time: Some(chrono::Utc::now()),
        });
        if i % 3 == rand % 3 {
            guard.add_pinned_message(nadir_types::model::Message {
                id: format!("BAR{}", rand % 5).into(),
                counter: None,
                tags: vec![
                    format!("bar{}", rand % 5),
                    "zenith".into(),
                    format!("al #{}", i),
                ],
                body: "汪汪汪汪汪汪汪".into(),
                time: Some(chrono::Utc::now()),
            });
        }
        handle.send(Box::new(|_c| {})).unwrap();
    }

    // handle.send().unwrap();
}

async fn time_update_loop(handle: CursiveHandle) -> ! {
    let mut timer = tokio::time::interval(std::time::Duration::from_millis(100));
    let mut time = chrono::Local::now();
    loop {
        timer.tick().await;
        let new_time = chrono::Local::now();
        if new_time.timestamp() / 60 != time.timestamp() / 60 {
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

    views::ResizedView::new(
        SizeConstraint::AtLeast(1),
        SizeConstraint::Fixed(1),
        views::LinearLayout::horizontal()
            .child(stat_view)
            .child(views::PaddedView::new(Margins::lr(1, 1), bar))
            .child(time_view),
    )
}

fn format_current_time(time: chrono::DateTime<Local>) -> String {
    let time = time.format("%Y-%m-%d %H:%M");
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

fn build_body(data: GroupRef) -> impl View {
    let mut view = GroupView::new(data, 7, false);

    view
}

// view::tag_view::TagView::new(
//     false,
//     1,
//     Style::default(),
//     vec!["西点子 DD 群111".into(), "Mad0ka".into()],
//     BracketConfig {
//         left: view::tag_view::BracketStyle::Square,
//         right: view::tag_view::BracketStyle::Angle,
//     },
//     "喵喵喵，喵喵喵念念念念念".into(),
//     DateTime::parse_from_rfc3339("2021-05-15T18:02:02+08:00")
//         .unwrap()
//         .with_timezone(&Local),
// ),

pub const LOGO: &str = r"
               _ _     
  _ _  __ _ __| (_)_ _ 
 | ' \/ _` / _` | | '_|
 |_||_\__,_\__,_|_|_|  
";
