use serde::{Serialize, Deserialize};

/// Represents a length which can be pixel-based or fractional.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Length {
    /// Pixel-based length.
    #[serde(rename = "px")]
    Pixel {
        value: i32,
    },
    /// Fractional length.
    #[serde(rename = "/")]
    Fractional {
        numerator: i32,
        denominator: i32,
    },
}
