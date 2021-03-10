
use crate::helpers::*;

// 'all' | 'md' | 'lg' | 'xl' | '2xl'
#[derive(Clone, PartialEq)]
pub enum ToolbarBreakpoint
{
    None,
    All,
    Md,
    Lg,
    Xl,
    Xxl,
}

pub fn get_toolbar_breakpoint(toolbar_breakpoint: &ToolbarBreakpoint) -> Option<i32>
{
    match toolbar_breakpoint
    {
        ToolbarBreakpoint::Md => Some(BREAKPOINT_MD),
        ToolbarBreakpoint::Lg => Some(BREAKPOINT_LG),
        ToolbarBreakpoint::Xl => Some(BREAKPOINT_XL),
        ToolbarBreakpoint::Xxl => Some(BREAKPOINT_XXL),
        _ => None,
    }
}