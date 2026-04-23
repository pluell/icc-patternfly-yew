use yew::prelude::*;


pub struct InnerScrollContainer;

#[derive(Clone, PartialEq, Properties)]
pub struct InnerScrollContainerProps
{
    /** Content rendered inside the inner scroll container */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the container */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for InnerScrollContainer
{
    type Message = ();
    type Properties = InnerScrollContainerProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    ctx.props().classes.clone(),
                    "pf-v5-c-scroll-inner-wrapper"
                )} 
                // {...props}
            >
            { ctx.props().children.clone() }
            </div>
        }
    }
}