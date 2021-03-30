
// These values are imported from the patternfly.css and may need
// to be updated in the future if the CSS file is updated
pub const BREAKPOINT_SM: i32 = 576;
pub const BREAKPOINT_MD: i32 = 768;
pub const BREAKPOINT_LG: i32 = 992;
pub const BREAKPOINT_XL: i32 = 1200;
pub const BREAKPOINT_XXL: i32 = 1450;

pub enum KeyCodes
{ 
    Tab = 9,
    Enter = 13,
    EscapeKey = 27,
    Space = 32,
    ArrowUp = 38,
    ArrowDown = 40,
}

#[derive(Clone, PartialEq)]
pub enum ValidatedOptions
{
    Success,
    Warning,
    Error,
    Default,
}