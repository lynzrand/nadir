use super::*;
use maildir::{Maildir, MailEntry};
use chrono::prelude::*;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct MaildirLocalServer {
    group_server: GroupServer,
    path: String,
}

impl MaildirLocalServer {
    pub fn new(group_server: GroupServer, path: String) -> Self {
        Self {
            group_server,
            path,
        }
    }

    fn single_serve(&self) {
        let mut group = self.group_server.data.write();
        group.msgs.clear(); // clear stale entries
        let mut display_entries = Vec::<(i64, MailEntry)>::new();
        let maildir = Maildir::from(self.path.clone());

        //for entry in maildir.list_cur() {
        //    let mut entry = entry.unwrap();
        //    let ts = entry.date().unwrap_or_else(|_| 0);
        //    if !entry.is_seen() {
        //        display_entries.push((ts, entry));
        //    }
        //}
        for entry in maildir.list_new() {
            let mut entry = entry.unwrap();
            let ts = entry.date().unwrap_or_else(|_| 0);
            display_entries.push((ts, entry));
        }

        display_entries.sort_by(|a, b| a.0.cmp(&b.0));

        for mut entry in display_entries.into_iter() {
            let mut entry = entry.1;
            let id = entry.id().to_string();
            let timestamp = entry.date().unwrap_or_else(|_| 0);
            if let Ok(headers) = entry.headers() {
                let subject = headers.iter()
                                    .filter(|x| x.get_key() == "Subject")
                                    .map(|x| x.get_value())
                                    .next()
                                    .unwrap_or_else(|| "No Subject".to_string());
                let from = headers.iter()
                                    .filter(|x| x.get_key() == "From")
                                    .map(|x| x.get_value())
                                    .next()
                                    .unwrap_or_else(|| "".to_string())
                                    .chars()
                                    // at most 16 char
                                    .take(16)
                                    .collect();
                group.add_message(nadir_types::model::Message {
                    id,
                    counter: None,
                    tags: vec![from],
                    body: subject,
                    time: Some(Utc.timestamp(timestamp, 0)),
                });
            }
        }

        self.group_server.handle
            .send(Box::new(|c| c.on_event(cursive::event::Event::Refresh)))
            .unwrap();
    }

    fn notify_serve(&self) {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
        watcher.watch(self.path.clone(), RecursiveMode::Recursive).unwrap();
        loop {
            if let Ok(_) = rx.recv() {
                self.single_serve();
            }
        }
    }
}

#[async_trait]
impl Server for MaildirLocalServer {
    async fn serve(self) {
        std::thread::spawn(move || {
            // init, then watch
            self.single_serve();
            self.notify_serve();
        });
    }
}
