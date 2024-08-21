use yew::prelude::*;


pub struct InnerScrollContainer;

#[derive(Clone, PartialEq, Properties)]
pub struct InnerScrollContainerProps
{
    /** Content rendered inside the inner scroll container */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the container */
    #[prop_or_default]
    pub class_name: String,
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
                    &ctx.props().class_name,
                    "pf-v5-c-scroll-inner-wrapper"
                )} 
                // {...props}
            >
            { for ctx.props().children.iter() }
            </div>
        }
    }
}