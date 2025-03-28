use web_sys::HtmlInputElement;
use yew::{
    prelude::*,
    events::InputEvent,
    html,
    Component, Context, Html, TargetCast,
};

use super::TextInputType;
use crate::ValidatedOptions;
use crate::components::{FormControlIcon, FromControlIconStatus};


#[derive(Clone, PartialEq)]
pub enum TextInputReadOnlyVariant
{
    Default,
    Plain,
}


pub struct TextInput {
    input_ref: NodeRef,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextInputProperties
{
    #[prop_or_default]
    pub id: AttrValue,
    /** Additional classes added to the TextInput. */
    #[prop_or_default]
    pub classes: Classes,
    /** Flag to show if the input is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    // /** Prop to apply expanded styling to the text input and link it to the element it is controlling. This should be used when the input controls a menu and that menu is expandable. */
    // expandedProps?: TextInputExpandedObj;
    /** Sets the input as readonly and determines the readonly styling. */
    #[prop_or_default]
    pub read_only_variant: Option<TextInputReadOnlyVariant>,
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
    pub value: AttrValue,  // | number;
    /** Placeholder of the text input when there is no value */
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /** Aria-label. The input requires an associated id or aria-label. */
    #[prop_or_default]
    pub aria_label: AttrValue,
    // /** A reference object to attach to the input box. */
    #[prop_or_default]
    pub inner_ref: Option<NodeRef>,
    /** Trim text on left */
    #[prop_or_default]
    pub is_left_truncated: bool,
    /** Trim text at start */
    #[prop_or_default]
    pub is_start_truncated: bool,
    // /** Callback function when input is focused */
    // #[prop_or_default]
    // onFocus?: (event?: any) => void;
    // /** Callback function when input is blurred (focus leaves) */
    // #[prop_or_default]
    // onBlur?: (event?: any) => void;
    // /** icon variant */
    // #[prop_or_default]
    // iconVariant?: 'calendar' | 'clock' | 'search';
    /** Custom icon to render. If the validated prop is also passed, this will render an icon in addition to a validated icon. */
    #[prop_or_default]
    pub custom_icon: Option<Html>,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<AttrValue>,
    /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    #[prop_or_default]
    pub ouia_safe: bool,
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
        Self {
            input_ref: NodeRef::default()
        }
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
        let has_status_icon = ctx.props().validated != ValidatedOptions::Default;

        let oninput = ctx.link().batch_callback(|e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| TextInputMsg::OnInput(input.value()))
        });

        html!{
            <span
                class={classes!(
                    "pf-v5-c-form-control",
                    if ctx.props().read_only_variant.is_some() {"pf-m-readonly"} else {""},
                    if ctx.props().read_only_variant == Some(TextInputReadOnlyVariant::Plain) {"pf-m-plain"} else {""},
                    if ctx.props().is_disabled {"pf-m-disabled"} else {""},
                    // (isExpanded || expandedProps?.isExpanded) && styles.modifiers.expanded, // TODO: implement isExpanded
                    if ctx.props().custom_icon.is_some() {"pf-m-icon"} else {""},
                    if has_status_icon {format!("pf-m-{}", ctx.props().validated.to_string())} else {String::new()},
                    ctx.props().classes.clone()
                )}
            >
                <input
                    // {...props}
                    id={ctx.props().id.clone()}
                    // onFocus={this.onFocus}
                    // onBlur={this.onBlur}
                    {oninput} // onChange={this.handleChange}
                    type={ctx.props().input_type.to_string()}
                    value={ctx.props().value.clone()}
                    aria-invalid={(ctx.props().validated == ValidatedOptions::Error).to_string()}
                    // {...ariaExpandedProps}
                    required={ctx.props().is_required}
                    disabled={ctx.props().is_disabled}
                    readonly={ctx.props().read_only_variant.is_some()}
                    ref={ctx.props().inner_ref.as_ref().unwrap_or(&self.input_ref).clone()}// {innerRef || this.inputRef}
                    placeholder={ctx.props().placeholder.clone()}
                    // {...getOUIAProps(TextInput.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId, ouiaSafe)}
                />
                {
                    if has_status_icon || ctx.props().custom_icon.is_some() {
                        html!{
                            <span class="pf-v5-c-form-control__utilities">
                            {
                                if let Some(custom_icon) = &ctx.props().custom_icon {
                                    html!{
                                        <FormControlIcon custom_icon={custom_icon.clone()} />
                                    }
                                } else {
                                    html!{}
                                }
                            }
                            {
                                html!{
                                    if has_status_icon {
                                        <FormControlIcon status={match ctx.props().validated {
                                            ValidatedOptions::Success => Some(FromControlIconStatus::Success),
                                            ValidatedOptions::Warning => Some(FromControlIconStatus::Warning),
                                            ValidatedOptions::Error => Some(FromControlIconStatus::Error),
                                            ValidatedOptions::Default => None,
                                        }} />
                                    }
                                }
                            }
                            </span>
                        }
                    } else {
                        html!{}
                    }
                }
            </span>
        }
    }
}
