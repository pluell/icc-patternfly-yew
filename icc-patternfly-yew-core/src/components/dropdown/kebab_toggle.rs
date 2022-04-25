
use yew::{
    prelude::*,
};

use crate::{ButtonType};

use super::{Toggle};


pub struct KebabToggle;

#[derive(Clone, PartialEq, Properties)]
pub struct KebabToggleProperties
{
    /** HTML ID of dropdown toggle */
    #[prop_or_default]
    pub id: String,
    /** Anything which can be rendered as dropdown toggle */
    #[prop_or_default]
    pub children: Children,
    /** Classess applied to root element of dropdown toggle */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to indicate if menu is opened */
    #[prop_or_default]
    pub is_open: bool,
    /** Label Toggle button */
    #[prop_or_default]
    pub aria_label: String,
    /** Callback called when toggle is clicked */
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
    // /** Element which wraps toggle */
    // parentRef: any;
    // /** The menu element */
    #[prop_or_default]
    pub menu_ref: NodeRef,
    /** Forces active state */
    #[prop_or_default]
    pub is_active: bool,
    /** Disables the dropdown toggle */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Display the toggle with no border or background */
    #[prop_or_default]
    pub is_plain: bool,
    /** Type to put on the button */
    #[prop_or(ButtonType::Button)]
    pub toggle_type: ButtonType,
    /** Allows selecting toggle to select parent */
    #[prop_or_default]
    pub bubble_event: bool,
    /** Whether or not the dropdown toggle button should have primary button styling */
    #[prop_or_default]
    pub is_primary: bool,

    #[prop_or_default]
    pub aria_haspopup: String,
}

impl Component for KebabToggle
{
    type Message = ();
    type Properties = KebabToggleProperties;

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
                aria_label={ctx.props().aria_label.clone()}
                menu_ref={ctx.props().menu_ref.clone()}
                is_active={ctx.props().is_active}
                is_disabled={ctx.props().is_disabled}
                is_plain={ctx.props().is_plain}
                is_primary={ctx.props().is_primary}
                ontoggle={ctx.props().ontoggle.clone()}
                bubble_event={ctx.props().bubble_event}
            >
                <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
            </Toggle>
        }
    }
}
