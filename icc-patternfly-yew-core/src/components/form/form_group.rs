use yew::{
    prelude::*,
};

use crate::{ASTERISK, ValidatedOptions};


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
    /** Sets an icon for the label. For providing additional context. Host element for Popover  */
    // #[prop_or_default]
    // labelIcon?: React.ReactElement;
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
    /** Removes top spacer from label. */
    #[prop_or_default]
    pub has_no_padding_top: bool,
    /** Helper text regarding the field. It can be a simple text or an object. */
    #[prop_or_default]
    pub helper_text: String,
    /** Flag to position the helper text before the field. False by default */
    #[prop_or_default]
    pub is_helper_text_before_field: bool,
    /** Helper text after the field when the field is invalid. It can be a simple text or an object. */
    #[prop_or_default]
    pub helper_text_invalid: String,
    /** Icon displayed to the left of the helper text. */
    // helperTextIcon?: React.ReactNode;
    /** Icon displayed to the left of the helper text when the field is invalid. */
    // helperTextInvalidIcon?: React.ReactNode;
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
                class=("pf-c-form__group", self.props.class_name.to_string())
            >
            {
                if !self.props.label.is_empty()
                {
                    html!{
                        <div
                            class=(
                                "pf-c-form__group-label",
                                if self.props.has_no_padding_top {"pf-m-no-padding-top"} else {""},
                            )
                        >
                            <label class="pf-c-form__label" for=self.props.field_id>
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
                            // {React.isValidElement(labelIcon) && labelIcon}
                        </div>
                    }
                }
                else
                {
                    html!{}
                }
            }
                <div 
                    class=(
                        "pf-c-form__group-control", 
                        if self.props.is_inline {"pf-m-inline"} else {""},
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
    fn get_helper_text(&self) -> Html
    {
        if self.props.validated != ValidatedOptions::Error
        {
            // let valid_text_helper = if !self.props.helper_text.is_empty() {
            if !self.props.helper_text.is_empty() {
                html!{
                    <div
                        class=(
                            "pf-c-form__helper-text",
                            if self.props.validated == ValidatedOptions::Success { "pf-m-success" } else { "" },
                            if self.props.validated == ValidatedOptions::Warning { "pf-m-warning" } else { "" },
                        )
                        id=format!("{}-helper", self.props.field_id)
                        aria-live="polite"
                    >
                        // {helperTextIcon && <span className={css(styles.formHelperTextIcon)}>{helperTextIcon}</span>}
                        {&self.props.helper_text}
                    </div>
                }
            } else {
                html!{}
            }
        }
        else
        {
            if !self.props.helper_text_invalid.is_empty()
            {
                html!{
                    <div 
                        class=(
                            "pf-c-form__helper-text",
                            "pf-m-error"
                        )
                             id=format!("{}-helper", self.props.field_id)
                         aria-live="polite"
                    >
                        // {helperTextInvalidIcon && <span className={css(styles.formHelperTextIcon)}>{helperTextInvalidIcon}</span>}
                        {&self.props.helper_text_invalid}
                    </div>
                }
            }
            else
            {
                html!{}
            }
        }
    }
}
