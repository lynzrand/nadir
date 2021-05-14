use cursive::{
    theme::{BaseColor::*, Color::*, ColorStyle, Palette, PaletteColor, PaletteColor::*},
    view::{Margins, Selector},
    views::{self, TextView},
};

#[tokio::main]
async fn main() {
    let mut siv = cursive::default();
    let theme = make_theme();
    siv.set_theme(theme);

    siv.add_fullscreen_layer(views::Layer::new(views::ResizedView::with_full_screen(
        views::LinearLayout::vertical()
            .child(views::PaddedView::new(Margins::tb(0, 1), make_stat()))
            .child(views::TextView::new("Hello World!\nThis is a sample text")),
    )));
    let handle = siv.cb_sink().clone();

    tokio::spawn(async move {
        let mut timer = tokio::time::interval(std::time::Duration::from_millis(250));
        loop {
            timer.tick().await;
            handle
                .send(Box::new(|c| {
                    c.call_on(&Selector::Name("time"), |s: &mut TextView| {
                        s.set_content(make_time());
                    });
                }))
                .unwrap();
        }
    });

    tokio::task::block_in_place(|| siv.run());
}

fn make_stat() -> impl cursive::View {
    let stat = "NOMINAL";
    let time = make_time();

    let stat_view = views::TextView::new(stat).style(ColorStyle::front(Light(Green)));
    let bar = views::TextView::new("|").style(ColorStyle::front(Light(Black)));
    let time_view = views::NamedView::new("time", views::TextView::new(time));

    views::LinearLayout::horizontal()
        .child(stat_view)
        .child(views::PaddedView::new(Margins::lr(1, 1), bar))
        .child(time_view)
}

fn make_time() -> String {
    let time = chrono::Local::now();
    let time = time.format("%Y-%m-%d %H:%M:%S");
    time.to_string()
}

fn make_theme() -> cursive::theme::Theme {
    let mut theme = cursive::theme::Theme::default();
    let palette = make_palette();

    theme.borders = cursive::theme::BorderStyle::None;
    theme.palette = palette;

    theme
}

fn make_palette() -> Palette {
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
