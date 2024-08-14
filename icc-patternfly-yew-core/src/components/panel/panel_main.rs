
use yew::prelude::*;


pub struct PanelMain;

#[derive(Clone, PartialEq, Properties)]
pub struct PanelMainProperties
{
    /** Content rendered inside the panel main div */
    #[prop_or_default]
    pub children: Children,
    /** Class to add to outer div */
    #[prop_or_default]
    pub classes: Classes,
    /** Max height of the panel main div as a string with the value and unit */
    #[prop_or_default]
    pub max_height: Option<String>,
}

impl Component for PanelMain
{
    type Message = ();
    type Properties = PanelMainProperties;

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
        // Set the style if width is set
        let style = if let Some(max_height) = &ctx.props().max_height {
            Some(format!("max-height: {}", max_height))
        } else {
            None
        };

        html!{
            <div
                class={classes!(        
                    "pf-v5-c-panel__main",
                    ctx.props().classes.clone(),        
                )}
                {style}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}