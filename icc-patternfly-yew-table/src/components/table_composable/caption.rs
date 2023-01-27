use yew::{
    prelude::*,
};


pub struct Caption;

#[derive(Clone, PartialEq, Properties)]
pub struct CaptionProps
{
    /** Content rendered inside the caption */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the caption  */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for Caption
{
    type Message = ();
    type Properties = CaptionProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <caption 
                class={classes!(
                    &ctx.props().class_name
                )}
                // {...props}
            >
            { for ctx.props().children.iter() }
            </caption>
        }
    }
}