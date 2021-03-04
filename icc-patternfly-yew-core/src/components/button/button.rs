use yew::{
    prelude::*,
};

use super::*;

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

const BTN_TYPES: &'static [&'static str] = &[
    "button",
    "submit",
    "reset",
];

pub struct Button
{
    props: ButtonProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProperties
{
    pub variant: ButtonVariant,
    #[prop_or(ButtonType::Button)]
    pub btn_type: ButtonType,
    #[prop_or_default]
    pub is_active: bool,
    #[prop_or_default]
    pub is_disabled: bool,
    #[prop_or_default]
    pub is_loading: Option<bool>,
    #[prop_or_default]
    pub is_block: bool,
    #[prop_or_default]
    pub is_small: bool,
    #[prop_or_default]
    pub is_large: bool,
    #[prop_or_default]
    pub is_inline: bool,
    #[prop_or_default]
    pub class_name: String,
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

impl Component for Button
{
    type Message = ();
    type Properties = ButtonProperties;

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
                class=(
                    "pf-c-button",
                    BTN_VARIANT_STYLES[self.props.variant.clone() as usize],
                    if self.props.is_block {"pf-m-block"} else {""},
                    if self.props.is_disabled {"pf-m-disabled"} else {""},
                    if self.props.is_active {"pf-m-active"} else {""},
                    // TODO: Implement inline
                    cls_loading,
                    if self.props.is_small {"pf-m-small"} else {""},
                    if self.props.is_large {"pf-m-display-lg"} else {""},
                    if self.props.is_inline && self.props.variant == ButtonVariant::Link {"pf-m-inline"} else {""},
                    self.props.class_name.clone(),
                )
                onclick=self.props.onclick.clone()
                disabled=self.props.is_disabled
                type=BTN_TYPES[self.props.btn_type.clone() as usize]
                role="button"
            >
            {
                if let Some(is_loading) = self.props.is_loading
                {
                    match is_loading
                    {
                        true => {
                            html!{
                                <span class="pf-c-button__progress">
                                    // TODO: implement Spinner component
                                    // <Spinner size={spinnerSize.md} aria-valuetext={spinnerAriaValueText} />
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
            // TODO: Implement icon left
            { self.props.children.clone() }
            // TODO: Implement icon right
            </button>
        }
    }
}
