use yew::{
    prelude::*,
};

use super::*;
use crate::{Spinner, SpinnerSize};

const BTN_VARIANT_STYLES: &'static [&'static str] = &[
    "pf-m-primary",
    "pf-m-secondary",
    "pf-m-tertiary",
    "pf-m-danger",
    "pf-m-warning",
    "pf-m-link",
    "pf-m-plain",
    "pf-m-control",
];

pub struct Button
{
    props: ButtonProperties,
    button_ref: NodeRef,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProperties
{
    /** Content rendered inside the button */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the button */
    #[prop_or_default]
    pub class_name: String,
    // /** Sets the base component to render. defaults to button */
    // component?: React.ElementType<any> | React.ComponentType<any>;
    /** Adds active styling to button. */
    #[prop_or_default]
    pub is_active: bool,
    /** Adds block styling to button */
    #[prop_or_default]
    pub is_block: bool,
    /** Adds disabled styling and disables the button using the disabled html attribute */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Adds disabled styling and communicates that the button is disabled using the aria-disabled html attribute */
    #[prop_or_default]
    pub is_aria_disabled: bool,
    /** Adds progress styling to button */
    #[prop_or_default]
    pub is_loading: Option<bool>,
    /** Aria-valuetext for the loading spinner */
    #[prop_or_default]
    pub spinner_aria_value_text: String,
    // /** Events to prevent when the button is in an aria-disabled state */
    // inoperableEvents?: string[];
    /** Adds inline styling to a link button */
    #[prop_or_default]
    pub is_inline: bool,
    /** Sets button type */
    #[prop_or(ButtonType::Button)]
    pub btn_type: ButtonType,
    /** Adds button variant styles */
    pub variant: ButtonVariant,
    /** Sets position of the link icon */
    #[prop_or(ButtonIconPosition::Left)]
    pub icon_position: ButtonIconPosition,
    /** Adds accessible text to the button. */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Icon for the button. Usable by all variants except for plain. */
    #[prop_or_default]
    pub icon: Option<Html>,
    /** Sets the button tabindex. */
    #[prop_or_default]
    pub tab_index: Option<i32>,
    /** Adds small styling to the button */
    #[prop_or_default]
    pub is_small: bool,
    /** Adds large styling to the button */
    #[prop_or_default]
    pub is_large: bool,
    /** Adds danger styling to secondary or link button variants */
    #[prop_or_default]
    pub is_danger: bool,
    /** Forwarded ref */
    #[prop_or_default]
    pub inner_ref: Option<NodeRef>,

    #[prop_or_default]
    pub id: String,

    pub onclick: Callback<MouseEvent>,

    // Extra aria properties
    #[prop_or_default]
    pub aria_controls: Option<String>,
    #[prop_or_default]
    pub aria_expanded: Option<String>,
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
}

impl Component for Button
{
    type Message = ();
    type Properties = ButtonProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        let button_ref = props.inner_ref.clone().unwrap_or(NodeRef::default());

        Self {
            props,
            button_ref,
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
        let cls_loading = if let Some(is_loading) = self.props.is_loading {
            match is_loading
            {
                true => "pf-m-progress pf-m-in-progress",
                false => "pf-m-progress",
            }
        }
        else
        {
            ""
        };

        html!{
            <button
                id=self.props.id.clone()
                aria-disabled=(self.props.is_disabled || self.props.is_aria_disabled).to_string()
                aria-label=self.props.aria_label.clone()
                class=classes!(
                    "pf-c-button",
                    BTN_VARIANT_STYLES[self.props.variant.clone() as usize],
                    if self.props.is_block {"pf-m-block"} else {""},
                    if self.props.is_disabled {"pf-m-disabled"} else {""},
                    if self.props.is_aria_disabled {"pf-m-aria-disabled"} else {""},
                    if self.props.is_active {"pf-m-active"} else {""},
                    if self.props.is_inline && self.props.variant == ButtonVariant::Link {"pf-m-inline"} else {""},
                    if self.props.is_danger && 
                        (self.props.variant == ButtonVariant::Secondary || self.props.variant == ButtonVariant::Link) {"pf-m-danger"} else {""},
                    cls_loading,
                    if self.props.is_small {"pf-m-small"} else {""},
                    if self.props.is_large {"pf-m-display-lg"} else {""},
                    self.props.class_name.clone(),
                )
                onclick=self.props.onclick.clone()
                disabled=self.props.is_disabled
                type=BTN_TYPES[self.props.btn_type.clone() as usize]
                role="button"
                ref=self.button_ref.clone()
                aria-controls=self.props.aria_controls.clone()
                aria-expanded=self.props.aria_expanded.clone()
                aria-labelledby=self.props.aria_labelledby.clone()
            >
            {
                if let Some(is_loading) = self.props.is_loading
                {
                    match is_loading
                    {
                        true => {
                            html!{
                                <span class="pf-c-button__progress">
                                    <Spinner size=SpinnerSize::Md aria_valuetext=self.props.spinner_aria_value_text.clone() />
                                </span>
                            }
                        },
                        false => html!{},
                    }
                }
                else
                {
                    html!{}
                }
            }
            {
                if let Some(icon) = &self.props.icon
                {
                    if self.props.variant != ButtonVariant::Plain && self.props.icon_position == ButtonIconPosition::Left
                    {
                        html!{
                            <span 
                                class=classes!(
                                    "pf-c-button__icon",
                                    "pf-m-start"
                                )
                            >
                                {icon.clone()}
                            </span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                else
                {
                    html!{}
                }
                
            }
            { self.props.children.clone() }
            {
                if let Some(icon) = &self.props.icon
                {
                    if self.props.variant != ButtonVariant::Plain && self.props.icon_position == ButtonIconPosition::Right
                    {
                        html!{
                            <span 
                                class=classes!(
                                    "pf-c-button__icon",
                                    "pf-m-end"
                                )
                            >
                                {icon.clone()}
                            </span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                else
                {
                    html!{}
                }
                
            }
            </button>
        }
    }
}
