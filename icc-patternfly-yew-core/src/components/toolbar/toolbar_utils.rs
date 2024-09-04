
use yew::{Callback, NodeRef};

use crate::helpers::*;


#[derive(Clone, PartialEq)]
pub struct ToolbarContext
{
    pub is_expanded: bool,
    // pub toggle_is_expanded: Callback<()>,
    pub chip_group_content_ref: NodeRef,
    pub update_number_filters: Callback<(String, i32)>,
    pub number_of_filters: i32,
    // clearAllFilters?: () => void;
    // clearFiltersButtonText?: string;
    // showClearFiltersButton?: boolean;
    // toolbarId?: string;
    // customChipGroupContent?: React.ReactNode;
}

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