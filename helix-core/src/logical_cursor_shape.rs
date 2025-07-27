use serde::{Deserialize, Serialize};

/// A shape that the cursor can take logically.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogicalCursorShape {
    /// The cursor has a width of 0 and is always on the left side of the current character.
    Bar = 0,
    /// The cursor has a width of 1 and is "on" the current character.
    Block = 1,
}
