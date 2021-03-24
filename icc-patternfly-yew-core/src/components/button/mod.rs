mod button;

pub use button::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant
{
    Primary,
    Secondary,
    Tertiary,
    Danger,
    Warning,
    Link,
    Plain,
    Control,
}

#[derive(Clone, PartialEq)]
pub enum ButtonType
{
    Button,
    Submit,
    Reset,
}

pub const BTN_TYPES: &'static [&'static str] = &[
    "button",
    "submit",
    "reset",
];