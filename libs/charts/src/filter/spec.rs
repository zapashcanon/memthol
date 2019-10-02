//! Filter specification.

use super::*;

/// A filter specification.
///
/// Contains the following:
///
/// - an optional UID;
/// - a name;
/// - a color.
///
/// The UID is optional because the filter specification can belong the "catch all" line of charts.
/// It is made from the points that all filters miss.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSpec {
    /// Uid of the filter.
    ///
    /// None if the specification if for the catch-all filter.
    uid: LineUid,
    /// Name of the filter.
    name: String,
    /// Color of the filter.
    color: Color,
    /// True if the filter has been edited.
    ///
    /// This is only used by the client to keep track of which filters have been edited in the UI.
    edited: bool,
}
impl FilterSpec {
    /// Constructor for user-defined filters.
    pub fn new(color: Color) -> Self {
        let uid = FilterUid::fresh();
        let name = format!("filter {}", uid);
        Self {
            uid: LineUid::Filter(uid),
            name,
            color,
            edited: false,
        }
    }

    /// Constructs a specification for the catch-all filter.
    pub fn new_catch_all() -> Self {
        Self {
            uid: LineUid::CatchAll,
            name: "catch all".into(),
            color: Color::new(0x01, 0x93, 0xff),
            edited: false,
        }
    }

    /// Constructs a specification for the everything filter.
    pub fn new_everything() -> Self {
        Self {
            uid: LineUid::Everything,
            name: "everything".into(),
            color: Color::new(0xff, 0x66, 0x00),
            edited: false,
        }
    }

    /// UID accessor.
    pub fn uid(&self) -> LineUid {
        self.uid
    }

    /// Value of the `edited` flag.
    pub fn edited(&self) -> bool {
        self.edited
    }
    /// Sets the `edited` flag to true.
    pub fn set_edited(&mut self) {
        self.edited = true
    }
    /// Sets the `edited` flag to false.
    pub fn unset_edited(&mut self) {
        self.edited = false
    }

    /// Name accessor.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Name setter.
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into()
    }

    /// Color accessor.
    pub fn color(&self) -> &Color {
        &self.color
    }
    /// Color setter.
    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }
}
