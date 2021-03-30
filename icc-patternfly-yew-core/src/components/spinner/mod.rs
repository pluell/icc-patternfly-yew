mod spinner;

pub use spinner::*;

#[derive(Clone, PartialEq)]
pub enum SpinnerSize
{
    Sm,
    Md,
    Lg,
    Xl,
}