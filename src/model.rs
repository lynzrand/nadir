//! Type models and containers for this specific use
//!

use arcstr::ArcStr;

/// A group of message to show
pub struct MessageGroup {
    ns: ArcStr,
    notifications: lru::LruCache<ArcStr, nadir_types::Notification>,
}
