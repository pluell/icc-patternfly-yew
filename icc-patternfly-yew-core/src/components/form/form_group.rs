use yew::{
    prelude::*,
    virtual_dom::VChild,
};

use crate::{ASTERISK, ValidatedOptions};

use super::FormHelperText;


#[derive(Clone, PartialEq)]
pub enum FormHelperTextTypes
{
    String(String),
    FormHelperText(VChild<FormHelperText>),
}

pub struct FormGroup;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                //{...props} 
                class={classes!("pf-v5-c-form__group", ctx.props().class_name.clone())}
            >
            {
                if !ctx.props().label.is_empty()
                {
                    if let Some(label_info) = &ctx.props().label_info
                    {
                        html!{
                            <>
                                <div class="pf-v5-c-form__group-label-main">{self.get_label_content(ctx)}</div>
                                <div class="pf-v5-c-form__group-label-info">{label_info.clone()}</div>
                            </>
                        }
                    }
                    else
                    {
                        self.get_label_content(ctx)
                    }
                }
                else
                {
                    html!{}
                }
            }
                <div 
                    class={classes!(
                        "pf-v5-c-form__group-control", 
                        if ctx.props().is_inline {"pf-m-inline"} else {""},
                        if ctx.props().is_stack {"pf-m-stack"} else {""},
                    )}
                >
                    {
                        if ctx.props().is_helper_text_before_field
                        {
                            self.get_helper_text(ctx)
                        }
                        else
                        {
                            html!{}
                        }
                    }
                    {
                        ctx.props().children.clone() 
                    }
                    {
                        if !ctx.props().is_helper_text_before_field
                        {
                            self.get_helper_text(ctx)
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
    fn get_label_content(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-form__group-label",
                    if ctx.props().label_info.is_some() {"pf-m-info"} else {""},
                    if ctx.props().has_no_padding_top {"pf-m-no-padding-top"} else {""},
                )}
            >
                <label class="pf-v5-c-form__label" for={ctx.props().field_id.clone()}>
                    <span class="pf-v5-c-form__label-text">{&ctx.props().label}</span>
                    {
                        if ctx.props().is_required
                        {
                            html!{
                                <span class="pf-v5-c-form__label-required" aria-hidden="true">
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
                    if let Some(label_icon) = &ctx.props().label_icon
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

    fn get_helper_text(&self, ctx: &Context<Self>) -> Html
    {
        if ctx.props().validated != ValidatedOptions::Error
        {
            if let Some(helper_text) = &ctx.props().helper_text
            {
                match helper_text
                {
                    FormHelperTextTypes::String(helper_text_str) => {
                        html!{
                            <div
                                class={classes!(
                                    "pf-v5-c-form__helper-text",
                                    if ctx.props().validated == ValidatedOptions::Success { "pf-m-success" } else { "" },
                                    if ctx.props().validated == ValidatedOptions::Warning { "pf-m-warning" } else { "" },
                                )}
                                id={format!("{}-helper", ctx.props().field_id)}
                                aria-live="polite"
                            >
                                {
                                    if let Some(helper_text_icon) = &ctx.props().helper_text_icon
                                    {
                                        html!{
                                            <span class="pf-v5-c-form__helper-text-icon">
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
            if let Some(helper_text_invalid) = &ctx.props().helper_text_invalid
            {
                match helper_text_invalid
                {
                    FormHelperTextTypes::String(helper_text_invalid_str) => {
                        html!{
                            <div 
                                class={classes!(
                                    "pf-v5-c-form__helper-text",
                                    "pf-m-error"
                                )}
                                id={format!("{}-helper", ctx.props().field_id)}
                                aria-live="polite"
                            >
                                {
                                    if let Some(helper_text_invalid_icon) = &ctx.props().helper_text_invalid_icon
                                    {
                                        html!{
                                            <span class="pf-v5-c-form__helper-text-icon">
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
