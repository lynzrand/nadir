pub mod fronend;
pub mod model;
pub mod opt;
pub mod ui;
pub mod util;
pub mod view;

use std::sync::Arc;

use chrono::Local;

use model::{group_list::GroupList, MessageGroup};
use util::DirtyCheckLock;
use view::group_list_view::GroupListView;
use zi::{ComponentExt, ComponentLink};

#[tokio::main]
async fn main() {
    // No auto refreshing, use the handle to trigger updates
    // siv.set_fps(5);
    log::set_max_level(log::LevelFilter::Info);

    let data = Arc::new(DirtyCheckLock::new(GroupList::new()));

    // tokio::spawn(time_update_loop(handle.clone()));
    tokio::spawn(data_update_loop(data));
    // tokio::spawn(server::start_server(handle.clone(), data, "[::1]:18234"));

    tokio::task::spawn_blocking(|| {
        let mut app = zi::App::new(GroupListView::with(data));
        app.run_event_loop(zi::frontend::default().unwrap())
            .unwrap()
    })
    .await
    .unwrap();
}

/// Testing function for updateing data
async fn data_update_loop(
    data: Arc<DirtyCheckLock<GroupList>>,
    link: ComponentLink<GroupListView>,
) {
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
    link.send(());
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
        link.send(());
    }
}

async fn time_update_loop() -> ! {
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
    }
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
