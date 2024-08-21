use web_sys::HtmlInputElement;
use yew::{
    prelude::*,
    events::InputEvent,
    html,
    Component, Context, Html, TargetCast,
};

use super::TextInputType;
use crate::ValidatedOptions;


pub struct TextInput;

#[derive(Clone, PartialEq, Properties)]
pub struct TextInputProperties
{
    #[prop_or_default]
    pub id: String,
    /** Additional classes added to the TextInput. */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show if the input is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag to show if the input is read only. */
    #[prop_or_default]
    pub is_read_only: bool,
    /** Flag to show if the input is required. */
    #[prop_or_default]
    pub is_required: bool,
    /** Value to indicate if the input is modified to show that validation state.
     * If set to success, input will be modified to indicate valid state.
     * If set to error,  input will be modified to indicate error state.
     */
    #[prop_or(ValidatedOptions::Default)]
    pub validated: ValidatedOptions,
    /** A callback for when the input value changes. */
    #[prop_or_default]
    pub onchange: Callback<String>, // (value: string, event: React.FormEvent<HTMLInputElement>) => void;
    /** Type that the input accepts. */
    #[prop_or(TextInputType::Text)]
    pub input_type: TextInputType,
    // /** Value of the input. */
    #[prop_or_default]
    pub value: String,  // | number;
    /** Aria-label. The input requires an associated id or aria-label. */
    #[prop_or_default]
    pub aria_label: String,
    // /** A reference object to attach to the input box. */
    // #[prop_or_default]
    // innerRef?: React.RefObject<any>;
    /** Trim text on left */
    #[prop_or_default]
    pub is_left_truncated: bool,
    // /** Callback function when input is focused */
    // #[prop_or_default]
    // onFocus?: (event?: any) => void;
    // /** Callback function when input is blurred (focus leaves) */
    // #[prop_or_default]
    // onBlur?: (event?: any) => void;
    // /** icon variant */
    // #[prop_or_default]
    // iconVariant?: 'calendar' | 'clock' | 'search';
    // /** Custom icon url to set as the input's background-image */
    // #[prop_or_default]
    // customIconUrl?: string;
    // /** Dimensions for the custom icon set as the input's background-size */
    // #[prop_or_default]
    // customIconDimensions?: string;
}

pub enum TextInputMsg
{
    OnInput(String),
}

impl Component for TextInput
{
    type Message = TextInputMsg;
    type Properties = TextInputProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            TextInputMsg::OnInput(value) => {
                ctx.props().onchange.emit(value);
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let oninput = ctx.link().batch_callback(|e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| TextInputMsg::OnInput(input.value()))
        });

        html!{
            <input
                id={ctx.props().id.clone()}
                // {...props}
                // onFocus={this.onFocus}
                // onBlur={this.onBlur}
                class={classes!(
                    "pf-v5-c-form-control",
                    if ctx.props().validated == ValidatedOptions::Success { "pf-m-success" } else { "" },
                    if ctx.props().validated == ValidatedOptions::Warning { "pf-m-warning" } else { "" },
                    // ((iconVariant && iconVariant !== 'search') || customIconUrl) && styles.modifiers.icon,
                    // iconVariant && styles.modifiers[iconVariant],
                    ctx.props().class_name.clone(),
                )}
                {oninput}
                type={ctx.props().input_type.to_string()}
                value={ctx.props().value.clone()}
                aria-invalid={(ctx.props().validated == ValidatedOptions::Error).to_string()}
                required={ctx.props().is_required}
                disabled={ctx.props().is_disabled}
                read_only={ctx.props().is_read_only.to_string()}
                // ref={innerRef || this.inputRef}
                // {...((customIconUrl || customIconDimensions) && { style: customIconStyle })}
            />
        }
    }
}
