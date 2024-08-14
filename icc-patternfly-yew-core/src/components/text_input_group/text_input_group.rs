
use yew::prelude::*;

use super::TextInputGroupContext;


pub struct TextInputGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct TextInputGroupProperties
{
    /** Content rendered inside the text input group */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes applied to the text input group container */
    #[prop_or_default]
    pub classes: Classes,
    /** Adds disabled styling and a disabled context value which text input group main hooks into for the input itself */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag to indicate the toggle has no border or background */
    #[prop_or_default]
    pub is_plain: bool,
//   /** @hide A reference object to attach to the input box */
//   innerRef?: React.RefObject<any>;
}

impl Component for TextInputGroup
{
    type Message = ();
    type Properties = TextInputGroupProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool
    {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html         
    {
        html!{
            <ContextProvider<TextInputGroupContext> context={TextInputGroupContext{is_disabled: ctx.props().is_disabled}}>
                <div
                    // ref={textInputGroupRef}
                    class={classes!(
                        "pf-v5-c-text-input-group",
                        if ctx.props().is_disabled { "pf-m-disabled" } else { "" },
                        if ctx.props().is_plain { "pf-m-plain" } else { "" },
                        ctx.props().classes.clone(),
                    )}
                    // {...props}
                >
                    {for ctx.props().children.iter()}
                </div>
            </ContextProvider<TextInputGroupContext>>
        }
    }
}