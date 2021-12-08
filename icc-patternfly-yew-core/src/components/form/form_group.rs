use yew::{
    prelude::*,
    virtual_dom::{VChild},
};

use crate::{ASTERISK, ValidatedOptions};

use super::{FormHelperText};


#[derive(Clone, PartialEq)]
pub enum FormHelperTextTypes
{
    String(String),
    FormHelperText(VChild<FormHelperText>),
}

pub struct FormGroup
{
    props: FormGroupProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormGroupProperties
{
     /** Anything that can be rendered as FormGroup content. */
    pub children: Children,
    /** Additional classes added to the FormGroup. */
    #[prop_or_default]
    pub class_name: String,
    /** Label text before the field. */
    #[prop_or_default]
    pub label: String,
    /** Additional label information displayed after the label. */
    #[prop_or_default]
    pub label_info: Option<Html>,
    /** Sets an icon for the label. For providing additional context. Host element for Popover  */
    #[prop_or_default]
    pub label_icon: Option<Html>,
    /** Sets the FormGroup required. */
    #[prop_or_default]
    pub is_required: bool,
    /**
     * Sets the FormGroup validated. If you set to success, text color of helper text will be modified to indicate valid state.
     * If set to error, text color of helper text will be modified to indicate error state.
     * If set to warning, text color of helper text will be modified to indicate warning state.
     */
    #[prop_or(ValidatedOptions::Default)]
    pub validated: ValidatedOptions,
    /** Sets the FormGroup isInline. */
    #[prop_or_default]
    pub is_inline: bool,
    /** Sets the FormGroupControl to be stacked */
    #[prop_or_default]
    pub is_stack: bool,
    /** Removes top spacer from label. */
    #[prop_or_default]
    pub has_no_padding_top: bool,
    /** Helper text regarding the field. It can be a simple text or an object. */
    #[prop_or_default]
    pub helper_text: Option<FormHelperTextTypes>,
    /** Flag to position the helper text before the field. False by default */
    #[prop_or_default]
    pub is_helper_text_before_field: bool,
    /** Helper text after the field when the field is invalid. It can be a simple text or an object. */
    #[prop_or_default]
    pub helper_text_invalid: Option<FormHelperTextTypes>,
    /** Icon displayed to the left of the helper text. */
    #[prop_or_default]
    pub helper_text_icon: Option<Html>,
    /** Icon displayed to the left of the helper text when the field is invalid. */
    #[prop_or_default]
    pub helper_text_invalid_icon: Option<Html>,
    /** ID of the included field. It has to be the same for proper working. */
    #[prop_or_default]
    pub field_id: String,
}

impl Component for FormGroup
{
    type Message = ();
    type Properties = FormGroupProperties;

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
            <div 
                //{...props} 
                class=classes!("pf-c-form__group", self.props.class_name.clone())
            >
            {
                if !self.props.label.is_empty()
                {
                    if let Some(label_info) = &self.props.label_info
                    {
                        html!{
                            <>
                                <div class="pf-c-form__group-label-main">{self.get_label_content()}</div>
                                <div class="pf-c-form__group-label-info">{label_info.clone()}</div>
                            </>
                        }
                    }
                    else
                    {
                        self.get_label_content()
                    }
                }
                else
                {
                    html!{}
                }
            }
                <div 
                    class=classes!(
                        "pf-c-form__group-control", 
                        if self.props.is_inline {"pf-m-inline"} else {""},
                        if self.props.is_stack {"pf-m-stack"} else {""},
                    )
                >
                    {
                        if self.props.is_helper_text_before_field
                        {
                            self.get_helper_text()
                        }
                        else
                        {
                            html!{}
                        }
                    }
                    {
                        self.props.children.clone() 
                    }
                    {
                        if !self.props.is_helper_text_before_field
                        {
                            self.get_helper_text()
                        }
                        else
                        {
                            html!{}
                        }
                    }
                </div>
            </div>
        }
    }
}

impl FormGroup
{
    fn get_label_content(&self) -> Html
    {
        html!{
            <div
                class=classes!(
                    "pf-c-form__group-label",
                    if self.props.label_info.is_some() {"pf-m-info"} else {""},
                    if self.props.has_no_padding_top {"pf-m-no-padding-top"} else {""},
                )
            >
                <label class="pf-c-form__label" for=self.props.field_id.clone()>
                    <span class="pf-c-form__label-text">{&self.props.label}</span>
                    {
                        if self.props.is_required
                        {
                            html!{
                                <span class="pf-c-form__label-required" aria-hidden="true">
                                    {' '}
                                    {ASTERISK}
                                </span>
                            }
                        }
                        else
                        {
                            html!{}
                        }
                    }
                </label>{' '}
                {
                    if let Some(label_icon) = &self.props.label_icon
                    {
                        label_icon.clone()
                    }
                    else
                    {
                        html!{}
                    }
                }
            </div>
        }
    }

    fn get_helper_text(&self) -> Html
    {
        if self.props.validated != ValidatedOptions::Error
        {
            if let Some(helper_text) = &self.props.helper_text
            {
                match helper_text
                {
                    FormHelperTextTypes::String(helper_text_str) => {
                        html!{
                            <div
                                class=classes!(
                                    "pf-c-form__helper-text",
                                    if self.props.validated == ValidatedOptions::Success { "pf-m-success" } else { "" },
                                    if self.props.validated == ValidatedOptions::Warning { "pf-m-warning" } else { "" },
                                )
                                id=format!("{}-helper", self.props.field_id)
                                aria-live="polite"
                            >
                                {
                                    if let Some(helper_text_icon) = &self.props.helper_text_icon
                                    {
                                        html!{
                                            <span class="pf-c-form__helper-text-icon">
                                                {helper_text_icon.clone()}
                                            </span>
                                        }
                                    }
                                    else
                                    {
                                        html!{}
                                    }
                                }
                                {helper_text_str}
                            </div>
                        }
                    },
                    FormHelperTextTypes::FormHelperText(helper_text_child) => {
                        html!{
                            helper_text_child.clone()
                        }
                    }
                }
            } else {
                html!{}
            }
        }
        else
        {
            if let Some(helper_text_invalid) = &self.props.helper_text_invalid
            {
                match helper_text_invalid
                {
                    FormHelperTextTypes::String(helper_text_invalid_str) => {
                        html!{
                            <div 
                                class=classes!(
                                    "pf-c-form__helper-text",
                                    "pf-m-error"
                                )
                                    id=format!("{}-helper", self.props.field_id)
                                aria-live="polite"
                            >
                                {
                                    if let Some(helper_text_invalid_icon) = &self.props.helper_text_invalid_icon
                                    {
                                        html!{
                                            <span class="pf-c-form__helper-text-icon">
                                                {helper_text_invalid_icon.clone()}
                                            </span>
                                        }
                                    }
                                    else
                                    {
                                        html!{}
                                    }
                                }
                                {helper_text_invalid_str}
                            </div>
                        }
                    },
                    FormHelperTextTypes::FormHelperText(helper_text_invalid_child) => {
                        html!{
                            helper_text_invalid_child.clone()
                        }
                    }
                }
            }
            else
            {
                html!{}
            }
        }
    }
}
