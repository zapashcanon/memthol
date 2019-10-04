//! Handles the internals of label sharing.

// Macro defined in `crate::mem`.
new! {
    mod mem for Vec<String>
}

/// A label UID.
pub struct Uid {
    uid: mem::Uid,
}

/// Registers a list of labels and returns its UID.
pub fn add(labels: Vec<String>) -> Uid {
    let mut mem = mem::write();
    let uid = mem.get_uid(labels);
    Uid { uid }
}

/// Retrieves a list of labels from its UID.
pub fn get(uid: Uid) -> std::sync::Arc<Vec<String>> {
    let mem = mem::read();
    mem.get_elm(uid.uid)
}
