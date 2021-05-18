//! Type models and containers for this specific use
//!

use std::num::NonZeroUsize;

use hashlink::lru_cache::LruCache;
use nadir_types::model;
use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("LRU cache was supplied with a capacity of 0")]
pub struct ZeroCapacityError;

/// A group of message to show. This type should be put inside a `Arc<RwLock<_>>`
/// for convenience.
pub struct MessageGroup {
    meta: model::MessageGroup,
    counter: u64,

    pub msgs: LruCache<SmolStr, model::Message>,
    pub pinned_msgs: LruCache<SmolStr, model::Message>,
}

impl MessageGroup {
    /// Create a new message group with the given capacity and pinned capacity.
    pub fn new(meta: model::MessageGroup) -> MessageGroup {
        MessageGroup {
            counter: 0,
            msgs: LruCache::new(meta.capacity),
            pinned_msgs: LruCache::new(meta.pinned_capacity),
            meta,
        }
    }

    pub fn meta(&self) -> &model::MessageGroup {
        &self.meta
    }

    pub fn set_meta(&mut self, meta: model::MessageGroup) -> Result<(), ZeroCapacityError> {
        self.msgs.set_capacity(meta.capacity);
        self.pinned_msgs.set_capacity(meta.pinned_capacity);
        self.meta = meta;
        Ok(())
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

    pub fn id(&self) -> SmolStr {
        self.meta.id.clone()
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

fn make_nonzero_usize(i: usize) -> Result<NonZeroUsize, ZeroCapacityError> {
    NonZeroUsize::new(i).ok_or(ZeroCapacityError)
}
