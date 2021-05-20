use std::iter::once;

use indexmap::IndexMap;

use crate::view::group_view::GroupRef;

/// A list of message groups, sorted by their metadata.
#[derive(Debug, Default)]
pub struct GroupList {
    /// The mapping from group IDs to groups themselves, sorted by their
    /// importance and then their ID.
    map: IndexMap<String, (i32, GroupRef)>,
}

impl GroupList {
    /// Initialize an empty new group list
    pub fn new() -> Self {
        Self::default()
    }

    /// Sort all groups inside self.
    pub fn sort_self(&mut self) {
        self.map
            .sort_by(|k1, v1, k2, v2| v1.0.cmp(&v2.0).reverse().then(k1.cmp(k2)));
    }

    /// Insert a new group into this list, and then sort the groups to retain
    /// the order.
    ///
    /// To insert multiple groups at once, use [`add_groups`] so it gets less
    /// sorted.
    pub fn add_group(&mut self, group: GroupRef) {
        self.add_groups(once(group))
    }

    /// Insert many groups into this list, and then sort the groups to retain
    /// the order.
    pub fn add_groups(&mut self, group: impl Iterator<Item = GroupRef>) {
        self.map.extend(group.map(|g| {
            let guard = g.read(false);
            let id = guard.id().to_owned();
            let importance = guard.meta().importance;
            drop(guard);
            (id, (importance, g))
        }));
        self.sort_self();
    }

    pub fn remove_group(&mut self, group: impl AsRef<str>) -> Option<GroupRef> {
        self.map.shift_remove(group.as_ref()).map(|x| x.1)
    }

    pub fn remove_groups(&mut self, groups: impl Iterator<Item = impl AsRef<str>>) {
        for group in groups {
            self.map.swap_remove(group.as_ref());
        }
        self.sort_self();
    }

    pub fn get_group(&self, id: impl AsRef<str>) -> Option<&GroupRef> {
        self.map.get(id.as_ref()).map(|v| &v.1)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &GroupRef)> {
        self.map.iter().map(|(k, (_, v))| (k.as_str(), v))
    }
}
