
use yew::prelude::*;


#[derive(Clone, PartialEq)]
pub enum PanelVariant
{
    Raised,
    Bordered
}

pub struct Panel;

#[derive(Clone, PartialEq, Properties)]
pub struct PanelProperties
{
    /** Content rendered inside the panel */
    #[prop_or_default]
    pub children: Children,
    /** Class to add to outer div */
    #[prop_or_default]
    pub classes: Classes,
    /** Adds panel variant styles */
    #[prop_or_default]
    pub variant: Option<PanelVariant>, //'raised' | 'bordered';
    /** Flag to add scrollable styling to the panel */
    #[prop_or_default]
    pub is_scrollable: bool,
    // /** @hide Forwarded ref */
    // innerRef?: React.Ref<any>;
}

impl Component for Panel
{
    type Message = ();
    type Properties = PanelProperties;

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
                    "pf-v5-c-panel",
                    match ctx.props().variant {
                        Some(PanelVariant::Raised) => "pf-m-raised",
                        Some(PanelVariant::Bordered) => "pf-m-bordered",
                        None => ""
                    },
                    if ctx.props().is_scrollable { "pf-m-scrollable" } else { "" },
                    ctx.props().classes.clone(),        
                )}
                // ref={innerRef}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}