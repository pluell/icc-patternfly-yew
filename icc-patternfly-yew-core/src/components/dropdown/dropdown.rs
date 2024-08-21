use yew::prelude::*;

use super::{DropdownDirection, DropdownPosition, DropdownWithContext, DropdownItemTypes, DropdownToggleComponents};


pub struct Dropdown;

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownProperties
{
    /** Anything which can be rendered in a dropdown */
    #[prop_or_default]
    pub children: Children,
    /** Classes applied to root element of dropdown */
    #[prop_or_default]
    pub class_name: String,
    /** Array of DropdownItem nodes that will be rendered in the dropdown Menu list */
    #[prop_or_default]
    pub dropdown_items: Vec<DropdownItemTypes>,
    /** Flag to indicate if menu is opened */
    #[prop_or_default]
    pub is_open: bool,
    /** Display the toggle with no border or background */
    #[prop_or_default]
    pub is_plain: bool,
    /** Indicates where menu will be aligned horizontally */
    #[prop_or(DropdownPosition::Left)]
    pub position: DropdownPosition,
    /** Display menu above or below dropdown toggle */
    #[prop_or(DropdownDirection::Down)]
    pub direction: DropdownDirection,
    /** Flag to indicate if dropdown has groups */
    #[prop_or_default]
    pub is_grouped: bool,
    /** Toggle for the dropdown, examples: <DropdownToggle> or <DropdownToggleCheckbox> */
    // #[prop_or_default]
    pub toggle: DropdownToggleComponents,
    /** Function callback called when user selects item */
    #[prop_or_default]
    pub onselect: Callback<()>,
    /** Flag to indicate if the first dropdown item should gain initial focus, set false when adding
     * a specific auto-focus item (like a current selection) otherwise leave as true
     */
    #[prop_or_default]
    pub auto_focus: bool,
}

impl Component for Dropdown
{
    type Message = ();
    type Properties = DropdownProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let props = ctx.props().clone();
        
        html!{
            <DropdownWithContext ..props />
        }
    }
}
