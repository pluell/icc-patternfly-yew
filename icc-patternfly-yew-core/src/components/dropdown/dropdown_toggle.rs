
use yew::{
    prelude::*,
};

use crate::{ButtonType};

use super::{Toggle};


pub struct DropdownToggle;

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownToggleProperties
{
    /** HTML ID of dropdown toggle */
    #[prop_or_default]
    pub id: String,
    /** Anything which can be rendered as dropdown toggle button */
    #[prop_or_default]
    pub children: Children,
    /** Classes applied to root element of dropdown toggle button */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to indicate if menu is opened */
    #[prop_or_default]
    pub is_open: bool,
    /** Callback called when toggle is clicked */
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
    /** The menu element */
    #[prop_or_default]
    pub menu_ref: NodeRef,
    /** Forces active state */
    #[prop_or_default]
    pub is_active: bool,
    /** Display the toggle with no border or background */
    #[prop_or_default]
    pub is_plain: bool,
    /** Whether or not the <div> has a disabled state */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Whether or not the dropdown toggle button should have primary button styling */
    #[prop_or_default]
    pub is_primary: bool,
    /** An image to display within the dropdown toggle, appearing before any component children */
    #[prop_or_default]
    pub icon: Option<Html>,
    /** The icon to display for the toggle, appearing after any component children. Defaults to CaretDownIcon. Set to null to not show an icon. */
    #[prop_or(Some(html!{<i class="fas fa-caret-down" aria-hidden="true"></i>}))]
    pub toggle_indicator: Option<Html>,
    /** Elements to display before the toggle button. When included, renders the toggle as a split button. */
    #[prop_or_default]
    pub split_button_items: Children,
    /** Variant of split button toggle */
    // #[prop_or_default]
    // pub splitButtonVariant?: 'action' | 'checkbox';
    /** Accessible label for the dropdown toggle button */
    #[prop_or_default]
    pub aria_label: String,
    /** Accessibility property to indicate correct has popup */
    #[prop_or_default]
    pub aria_haspopup: String, // boolean | 'listbox' | 'menu' | 'dialog' | 'grid' | 'tree';
    /** Type to put on the button */
    #[prop_or(ButtonType::Button)]
    pub toggle_type: ButtonType,
    // /** Callback called when the Enter key is pressed */
    // pub onenter: (event?: React.MouseEvent<HTMLButtonElement>) => void;
}

impl Component for DropdownToggle
{
    type Message = ();
    type Properties = DropdownToggleProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <Toggle
                // {...props}
                id={ctx.props().id.clone()}
                class_name={ctx.props().class_name.clone()}
                is_open={ctx.props().is_open}
                menu_ref={ctx.props().menu_ref.clone()}
                is_active={ctx.props().is_active}
                is_disabled={ctx.props().is_disabled}
                is_plain={ctx.props().is_plain}
                is_primary={ctx.props().is_primary}
                ontoggle={ctx.props().ontoggle.clone()}
                aria_haspopup={ctx.props().aria_haspopup.clone()}
                toggle_type={ctx.props().toggle_type.clone()}
                // {...ouiaProps}
                // {...(splitButtonItems && { isSplitButton: true, 'aria-label': props['aria-label'] || 'Select' })}
            >
                {
                    if let Some(icon) = &ctx.props().icon
                    {
                        html!{
                            <span class="pf-c-dropdown__toggle-icon">{ icon.clone() }</span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                { 
                    if ctx.props().children.is_empty()
                    {
                        html!{}
                    }
                    else
                    {
                        html!{
                            <span class={if ctx.props().toggle_indicator.is_some() { "pf-c-dropdown__toggle-text" } else { "" }}>
                                { ctx.props().children.clone() }
                            </span>
                        }
                    }
                }
                {
                    if let Some(toggle_indicator) = &ctx.props().toggle_indicator
                    {
                        html!{
                            <span
                                class={if ctx.props().split_button_items.len() == 0 { "pf-c-dropdown__toggle-icon" } else { "" }}
                            >
                                { toggle_indicator.clone() }
                            </span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
            </Toggle>
        }
    }
}
