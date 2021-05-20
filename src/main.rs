pub mod model;
pub mod opt;
pub mod server;
pub mod ui;
pub mod util;
pub mod view;

use std::sync::Arc;

use chrono::{DateTime, Local};
use cursive::{
    event::Event,
    theme::{BaseColor::*, Color::*, ColorStyle, Palette, PaletteColor::*},
    view::{Margins, Selector, SizeConstraint},
    views::{self, DebugView, LinearLayout, ResizedView, TextView},
    Cursive, View,
};
use model::{group_list::GroupList, MessageGroup};
use util::DirtyCheckLock;
use view::{
    group_list_view::GroupListView,
    group_view::{GroupRef, GroupView},
};

pub type CursiveHandle = crossbeam::channel::Sender<Box<dyn FnOnce(&mut Cursive) + 'static + Send>>;

#[tokio::main]
async fn main() {
    let mut siv = cursive::default();
    let theme = init_theme();
    siv.set_theme(theme);
    // No auto refreshing, use the handle to trigger updates
    // siv.set_fps(5);
    cursive::logger::init();
    log::set_max_level(log::LevelFilter::Info);

    let data = Arc::new(DirtyCheckLock::new(GroupList::new()));

    siv.add_fullscreen_layer(views::Layer::new(views::ResizedView::with_full_screen(
        views::LinearLayout::vertical()
            .child(views::PaddedView::new(Margins::tb(0, 1), init_stat()))
            // .child(ResizedView::with_max_height(6, DebugView::new()))
            .child(build_body(data.clone())),
    )));
    siv.add_global_callback(Event::CtrlChar('d'), |c| c.toggle_debug_console());
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
async fn data_update_loop(handle: CursiveHandle, data: Arc<DirtyCheckLock<GroupList>>) {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let group_name = "tg".to_string();
    let group = Arc::new(DirtyCheckLock::new(MessageGroup::new(
        nadir_types::model::MessageGroup {
            id: group_name.clone(),
            title: "Telegram".into(),
            capacity: 30,
            pinned_capacity: 3,
            importance: 0,
        },
    )));
    {
        data.write().add_group(group.clone());
    }
    handle
        .send(Box::new(|c| c.on_event(cursive::event::Event::Refresh)))
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let group_name_2 = "maildir".to_string();
    let group_2 = Arc::new(DirtyCheckLock::new(MessageGroup::new(
        nadir_types::model::MessageGroup {
            id: group_name_2.clone(),
            title: "Maildir".into(),
            capacity: 30,
            pinned_capacity: 3,
            importance: -4,
        },
    )));
    {
        data.write().add_group(group_2.clone());
    }

    let mut i: u64 = 1;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        {
            let mut group = group.write();
            group.add_message(nadir_types::model::Message {
                id: format!("aaa{}", i % 24),
                counter: None,
                tags: vec![format!("foo{}", i % 24)],
                body: format!("{}", i),
                time: Some(chrono::Utc::now()),
            });
        }

        {
            let mut group = group_2.write();
            group.add_message(nadir_types::model::Message {
                id: format!("aaa{}", i % 17),
                counter: None,
                tags: vec![format!("foo{}", i % 17)],
                body: format!("{}", i),
                time: Some(chrono::Utc::now()),
            });
            i = i.wrapping_add(i << 17).wrapping_add(i >> 13);
        }
        handle
            .send(Box::new(|c| c.on_event(cursive::event::Event::Refresh)))
            .unwrap();
    }
}

async fn time_update_loop(handle: CursiveHandle) -> ! {
    let mut timer = tokio::time::interval(std::time::Duration::from_millis(100));
    let mut time = chrono::Local::now();
    loop {
        timer.tick().await;
        let new_time = chrono::Local::now();
        // if new_time.timestamp() / 60 != time.timestamp() / 60 {
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
    let bar = views::PaddedView::new(
        Margins::lr(1, 1),
        views::TextView::new("|").style(ColorStyle::front(Light(Black))),
    );

    let bar2 = views::PaddedView::new(
        Margins::lr(1, 1),
        views::TextView::new("|").style(ColorStyle::front(Light(Black))),
    );

    let time_view = views::NamedView::new("time", views::TextView::new(time));

    let app_ver = views::TextView::new(format!("{} v{}", APP_NAME, APP_VER)).style(Secondary);

    views::ResizedView::new(
        SizeConstraint::AtLeast(1),
        SizeConstraint::Fixed(1),
        views::LinearLayout::horizontal()
            .child(stat_view)
            .child(bar)
            .child(time_view)
            .child(bar2)
            .child(app_ver),
    )
}

fn format_current_time(time: chrono::DateTime<Local>) -> String {
    let time = time.format("%Y-%m-%d %H:%M:%S");
    time.to_string()
}

fn init_theme() -> cursive::theme::Theme {
    let mut theme = cursive::theme::Theme::default();
    let palette = init_palette();

    // theme.borders = cursive::theme::BorderStyle::None;
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

fn build_body(data: Arc<DirtyCheckLock<GroupList>>) -> impl View {
    views::ResizedView::with_full_screen(GroupListView::new(data, Box::new(build_empty_view)))
}

fn build_empty_view() -> Box<dyn View> {
    Box::new(ResizedView::with_full_screen(
        cursive_aligned_view::AlignedView::new(
            views::LinearLayout::horizontal()
                .child(views::PaddedView::lrtb(
                    3,
                    3,
                    3,
                    3,
                    TextView::new(NADIR_LOGO),
                ))
                .child(views::PaddedView::lrtb(
                    3,
                    3,
                    3,
                    3,
                    LinearLayout::vertical()
                        .child(TextView::new(NADIR_NAME))
                        .child(views::PaddedView::lrtb(
                            1,
                            0,
                            1,
                            0,
                            TextView::new("Waiting for connections"),
                        )),
                )),
            cursive::align::Align::center(),
        ),
    ))
}

pub const NADIR_NAME: &str = r"
               _ _     
  _ _  __ _ __| (_)_ _ 
 | ' \/ _` / _` | | '_|
 |_||_\__,_\__,_|_|_|  
";

pub const NADIR_LOGO: &str = r"
\--                           ---/
 \-----                  ----===/ 
   \--------------------=====//   
     -\===--------=======////-    
        \\===========//////       
            \\\==*/////           
";

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VER: &str = env!("CARGO_PKG_VERSION");
