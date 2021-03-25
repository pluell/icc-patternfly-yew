
use yew::{
    prelude::*,
};

use crate::{ButtonType};

use super::{Toggle};


pub struct KebabToggle
{
    props: KebabToggleProperties,
}

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
}

impl Component for KebabToggle
{
    type Message = ();
    type Properties = KebabToggleProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            
            true
        }
        else
        {
            false
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <Toggle
                // {...props}
                id=self.props.id.clone()
                class_name=self.props.class_name.clone()
                is_open=self.props.is_open
                aria_label=self.props.aria_label.clone()
                menu_ref=self.props.menu_ref.clone()
                is_active=self.props.is_active
                is_disabled=self.props.is_disabled
                is_plain=self.props.is_plain
                is_primary=self.props.is_primary
                ontoggle=self.props.ontoggle.clone()
                bubble_event=self.props.bubble_event
            >
                <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
            </Toggle>
        }
    }
}
