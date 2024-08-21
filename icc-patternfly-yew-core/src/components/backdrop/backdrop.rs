use yew::prelude::*;


pub struct Backdrop;

#[derive(Clone, PartialEq, Properties)]
pub struct BackdropProperties
{
    /** content rendered inside the backdrop */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the button */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for Backdrop
{
    type Message = ();
    type Properties = BackdropProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-backdrop",
                    ctx.props().class_name.clone()
                )}
            >
            { for ctx.props().children.iter() }
            </div>
        }
    }
}
