use yew::{
    prelude::*,
};


pub struct EmptyStatePrimary;

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStatePrimaryProps
{
    /** Additional classes added to the EmptyStatePrimary */
    #[prop_or_default]
    pub class_name: String,
    /** Content rendered inside the EmptyStatePrimary */
    #[prop_or_default]
    pub children: Children,
}

impl Component for EmptyStatePrimary
{
    type Message = ();
    type Properties = EmptyStatePrimaryProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-c-empty-state__primary",
                    ctx.props().class_name.clone()
                )}
                // {...props}
            >
            {
                for ctx.props().children.iter()
            }
            </div>
        }
    }
}
