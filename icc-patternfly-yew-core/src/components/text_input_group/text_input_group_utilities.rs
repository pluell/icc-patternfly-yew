
use yew::prelude::*;


pub struct TextInputGroupUtilities;

#[derive(Clone, PartialEq, Properties)]
pub struct TextInputGroupUtilitiesProperties
{
    /** Content rendered inside the text input group utilities div */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes applied to the text input group utilities container */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for TextInputGroupUtilities
{
    type Message = ();
    type Properties = TextInputGroupUtilitiesProperties;

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
            <div
                // ref={textInputGroupRef}
                class={classes!(
                    "pf-v5-c-text-input-group__utilities",
                    ctx.props().classes.clone(),
                )}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}