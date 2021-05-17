//! Type models and containers for this specific use
//!

use std::num::NonZeroUsize;

use arcstr::ArcStr;
use clru::CLruCache;
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

    pub msgs: CLruCache<SmolStr, model::Message>,
    pub pinned_msgs: CLruCache<SmolStr, model::Message>,
}

impl MessageGroup {
    /// Create a new message group with the given capacity and pinned capacity.
    pub fn new(meta: model::MessageGroup) -> Result<MessageGroup, ZeroCapacityError> {
        let cap = make_nonzero_usize(meta.capacity)?;
        let cap_pinned = make_nonzero_usize(meta.pinned_capacity)?;
        Ok(MessageGroup {
            meta,
            counter: 0,
            msgs: CLruCache::new(cap),
            pinned_msgs: CLruCache::new(cap_pinned),
        })
    }

    pub fn meta(&self) -> &model::MessageGroup {
        &self.meta
    }

    pub fn set_meta(&mut self, meta: model::MessageGroup) -> Result<(), ZeroCapacityError> {
        self.msgs.resize(make_nonzero_usize(meta.capacity)?);
        self.pinned_msgs.resize(make_nonzero_usize(meta.capacity)?);
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
        self.msgs.put(notification.id.clone(), notification);
    }

    pub fn add_messages(&mut self, messages: impl Iterator<Item = model::Message>) {
        for msg in messages {
            self.msgs.put(msg.id.clone(), msg);
        }
    }

    pub fn remove_msg<'a>(&mut self, ids: impl Iterator<Item = &'a str>) {
        for id in ids {
            self.msgs.pop(id);
        }
    }
}

fn make_nonzero_usize(i: usize) -> Result<NonZeroUsize, ZeroCapacityError> {
    Ok(NonZeroUsize::new(i).ok_or(ZeroCapacityError)?)
}
