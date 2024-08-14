
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::TextInputGroupContext;
use crate::TextInputType;

pub struct TextInputGroupMain
{
    context: TextInputGroupContext,
    _context_listener: ContextHandle<TextInputGroupContext>
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextInputGroupMainProperties
{
    /** Content rendered inside the text input group main div */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes applied to the text input group main container */
    #[prop_or_default]
    pub classes: Classes,
    /** Icon to be shown on the left side of the text input group main container */
    #[prop_or_default]
    pub icon: Option<Html>,
    /** Type that the input accepts. */
    #[prop_or(TextInputType::Text)]
    pub input_type: TextInputType,
    /** Suggestion that will show up like a placeholder even with text in the input */
    #[prop_or_default]
    pub hint: Option<String>,
    /** Callback for when there is a change in the input field*/
    #[prop_or_default]
    pub onchange: Callback<String>, // onChange?: (event: React.FormEvent<HTMLInputElement>, value: string) => void;
    /** Callback for when the input field is focused*/
    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>, // onFocus?: (event?: any) => void;
    /** Callback for when focus is lost on the input field*/
    #[prop_or_default]
    pub onblur: Callback<FocusEvent>, // onBlur?: (event?: any) => void;
    /** Accessibility label for the input */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Value for the input */
    #[prop_or_default]
    pub value: String,
    /** Placeholder value for the input */
    #[prop_or_default]
    pub placeholder: Option<String>,
    // /** @hide A reference object to attach to the input box */
    // innerRef?: React.RefObject<any>;
    /** Name for the input */
    #[prop_or_default]
    pub name: String,
    // /** @beta The id of the active element. Required if role has a value of "combobox", and focus
    //  * should remain on the input.
    //  */
    // 'aria-activedescendant'?: string;
    // /** @beta Determines the accessible role of the input. */
    // role?: string;
    // /** @beta Flag for whether an associated element controlled by the input is visible. Required if
    //  * role has a value of "combobox".
    //  */
    // isExpanded?: boolean;
    // /** @beta The id of the element(s) controlled by the input. Required if role has a value of "combobox". */
    // 'aria-controls'?: string;
    /** The id of the input element */
    #[prop_or_default]
    pub input_id: String,       
}

pub enum TextInputGroupMainMessage
{
    Context(TextInputGroupContext),
    OnChange(String),
    OnBlur(FocusEvent),
    OnFocus(FocusEvent),
}

impl Component for TextInputGroupMain
{
    type Message = TextInputGroupMainMessage;
    type Properties = TextInputGroupMainProperties;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(TextInputGroupMainMessage::Context))
            .expect("No Message Context Provided");

        Self {
            context,
            _context_listener: context_listener,
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg : Self::Message) -> bool
    {
        match msg
        {
            TextInputGroupMainMessage::Context(context) => {
                self.context = context;
                true
            }
            TextInputGroupMainMessage::OnChange(value) => {
                ctx.props().onchange.emit(value);
                true
            }
            TextInputGroupMainMessage::OnBlur(event) => {
                ctx.props().onblur.emit(event);
                true
            }
            TextInputGroupMainMessage::OnFocus(event) => {
                ctx.props().onfocus.emit(event);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html         
    {
        let onchange = ctx.link().batch_callback(|e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| TextInputGroupMainMessage::OnChange(input.value()))
        });

        let onblur = ctx.link().batch_callback(|e: FocusEvent| {
            Some(TextInputGroupMainMessage::OnBlur(e))
        });

        let onfocus = ctx.link().batch_callback(|e: FocusEvent| {
            Some(TextInputGroupMainMessage::OnFocus(e))
        });

        html!{
            <div 
                class={classes!(
                    "pf-v5-c-text-input-group__main",
                    if ctx.props().icon.is_some() { "pf-m-icon" } else {""},
                    ctx.props().classes.clone(),
                )}
            >
                {for ctx.props().children.iter()}
                <span class="pf-v5-c-text-input-group__text">
                    {
                        if let Some(hint) = &ctx.props().hint
                        {
                            html!{
                                <input
                                    class="pf-v5-c-text-input-group__text-input"
                                    type="text"
                                    disabled=true
                                    aria-hidden="true"
                                    value={hint.clone()}
                                    id={ctx.props().input_id.clone()}
                                />
                            }
                        }
                        else
                        {
                            html!{}
                        }
                    }
                    {
                        if let Some(icon) = &ctx.props().icon
                        {
                            html!{<span class="pf-v5-c-text-input-group__icon">{icon.clone()}</span>}
                        }
                        else
                        {
                            html!{}
                        }
                    }
                    <input
                        // ref={textInputGroupInputInputRef}
                        type={ctx.props().input_type.to_string()}
                        class="pf-v5-c-text-input-group__text-input"
                        aria_label={ctx.props().aria_label.clone()}
                        disabled={self.context.is_disabled}
                        {onchange}
                        {onfocus}
                        {onblur}
                        value={ctx.props().value.clone()}
                        placeholder={ctx.props().placeholder.clone()}
                        name={ctx.props().name.clone()} 
                        // aria-activedescendant={ariaActivedescendant}
                        id={ctx.props().input_id.clone()}
                        // {...(role && { role })}
                        // {...(isExpanded !== undefined && { 'aria-expanded': isExpanded })}
                        // {...(ariaControls && { 'aria-controls': ariaControls })}
                    />
                </span>
            </div>
        }
    }
}