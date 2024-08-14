
use yew::prelude::*;


pub struct PanelHeader;

#[derive(Clone, PartialEq, Properties)]
pub struct PanelHeaderProperties
{
    /** Content rendered inside the panel header */
    #[prop_or_default]
    pub children: Children,
    /** Class to add to outer div */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for PanelHeader
{
    type Message = ();
    type Properties = PanelHeaderProperties;

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
                    "pf-v5-c-panel__header",
                    ctx.props().classes.clone(),        
                )}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}