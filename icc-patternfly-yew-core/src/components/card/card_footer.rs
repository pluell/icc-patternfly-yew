use yew::prelude::*;


pub struct CardFooter;

#[derive(Clone, PartialEq, Properties)]
pub struct CardFooterProperties
{
    /** Content rendered inside the Card Footer */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the Footer */
    #[prop_or_default]
    pub classes: Classes,
    /** Sets the base component to render. defaults to div */
    #[prop_or(String::from("div"))]
    pub component: String,
}

impl Component for CardFooter
{
    type Message = ();
    type Properties = CardFooterProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <@{ctx.props().component.to_string()}
                class={classes!(
                    "pf-v5-c-card__footer", 
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {ctx.props().children.clone()}
            </@>
        }
    }
}
