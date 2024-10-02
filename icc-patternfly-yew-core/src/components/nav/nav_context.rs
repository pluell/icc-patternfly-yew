use yew::prelude::*;

use super::{NavSelectClickHandler, NavSelectedItem};


pub struct NavContextOnSelectProps
{
    pub event: MouseEvent,
    pub selected_item: NavSelectedItem,
    pub prevent_default: bool,
    pub onclick: Option<NavSelectClickHandler>,
}

#[derive(Clone, PartialEq)]
pub struct  NavContext
{
    pub onselect: Callback<NavContextOnSelectProps>,
    pub ontoggle: Option<Callback<(String, bool)>>,
    pub update_is_scrollable: Callback<bool>,
    pub is_horizontal: bool,
    pub flyout_ref: Option<NodeRef>,
    // pub set_flyout_ref?: (ref: React.Ref<HTMLLIElement>) => void;
    pub nav_ref: NodeRef,
}