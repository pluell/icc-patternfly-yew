use yew::{
    prelude::*,
};


pub struct OuterScrollContainer;

#[derive(Clone, PartialEq, Properties)]
pub struct OuterScrollContainerProps
{
    /** Content rendered inside the outer scroll container */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the container */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for OuterScrollContainer
{
    type Message = ();
    type Properties = OuterScrollContainerProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    &ctx.props().class_name,
                    "pf-c-scroll-outer-wrapper"
                )} 
                // {...props}
            >
            { for ctx.props().children.iter() }
            </div>
        }
    }
}