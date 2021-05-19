use std::cmp::{min};

use hashlink::lru_cache::LruCache;
use nadir_types::model;

/// A hard maximum value for all messages to prevent memory overflow.
const CAPACITY_HARD_MAX: usize = 400;

/// A group of message to show. This type should be put inside a `Arc<RwLock<_>>`
/// to share between threads.
#[derive(Debug)]
pub struct MessageGroup {
    meta: model::MessageGroup,
    counter: u64,

    pub msgs: LruCache<String, model::Message>,
    pub pinned_msgs: LruCache<String, model::Message>,
}

impl MessageGroup {
    /// Create a new message group with the given capacity and pinned capacity.
    pub fn new(meta: model::MessageGroup) -> MessageGroup {
        MessageGroup {
            counter: 0,
            msgs: LruCache::new(min(meta.capacity as usize, CAPACITY_HARD_MAX)),
            pinned_msgs: LruCache::new(min(meta.pinned_capacity as usize, CAPACITY_HARD_MAX)),
            meta,
        }
    }

    pub fn meta(&self) -> &model::MessageGroup {
        &self.meta
    }

    pub fn set_meta(&mut self, meta: model::MessageGroup) {
        self.msgs
            .set_capacity(min(meta.capacity as usize, CAPACITY_HARD_MAX));
        self.pinned_msgs
            .set_capacity(min(meta.pinned_capacity as usize, CAPACITY_HARD_MAX));
        self.meta = meta;
    }

    pub fn set_counter(&mut self, counter: u64) {
        self.counter = counter;
    }

    pub fn inc_counter(&mut self, counter: i64) {
        if counter >= 0 {
            self.counter = self.counter.saturating_add(counter as u64);
        } else {
            self.counter = self.counter.saturating_sub((-counter) as u64);
        }
    }

    pub fn counter(&self) -> u64 {
        self.counter
    }

    pub fn id(&self) -> &str {
        &self.meta.id
    }

    pub fn cap(&self) -> usize {
        self.msgs.capacity()
    }

    pub fn cap_pinned(&self) -> usize {
        self.pinned_msgs.capacity()
    }

    pub fn add_message(&mut self, notification: model::Message) {
        self.msgs.insert(notification.id.clone(), notification);
    }

    pub fn add_messages(&mut self, messages: impl Iterator<Item = model::Message>) {
        for msg in messages {
            self.msgs.insert(msg.id.clone(), msg);
        }
    }

    pub fn add_pinned_message(&mut self, notification: model::Message) {
        self.pinned_msgs
            .insert(notification.id.clone(), notification);
    }

    pub fn add_pinned_messages(&mut self, messages: impl Iterator<Item = model::Message>) {
        for msg in messages {
            self.pinned_msgs.insert(msg.id.clone(), msg);
        }
    }

    pub fn remove_msg<'a>(&mut self, ids: impl Iterator<Item = &'a str>) {
        for id in ids {
            self.msgs.remove(id);
        }
    }

    pub fn remove_pinned_msg<'a>(&mut self, ids: impl Iterator<Item = &'a str>) {
        for id in ids {
            self.pinned_msgs.remove(id);
        }
    }
}
