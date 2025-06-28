#![allow(dead_code)]

#[derive(Clone, strum_macros::Display)]
pub enum Orientation {
    #[strum(to_string = "horizontal")]
    Horizontal,
    #[strum(to_string = "vertical")]
    Vertical,
}
