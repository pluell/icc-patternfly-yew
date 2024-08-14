
use yew::prelude::*;


pub struct PanelFooter;

#[derive(Clone, PartialEq, Properties)]
pub struct PanelFooterProperties
{
    /** Content rendered inside the panel footer */
    #[prop_or_default]
    pub children: Children,
    /** Class to add to outer div */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for PanelFooter
{
    type Message = ();
    type Properties = PanelFooterProperties;

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
                class={classes!(
                    "pf-v5-c-panel__footer  ",
                    ctx.props().classes.clone(),        
                )}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}