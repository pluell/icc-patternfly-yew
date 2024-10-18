use yew::prelude::*;


pub struct CardBody;

#[derive(Clone, PartialEq, Properties)]
pub struct CardBodyProperties
{
    /** Content rendered inside the Card Body */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the Card Body */
    #[prop_or_default]
    pub classes: Classes,
    /** Sets the base component to render. defaults to div */
    #[prop_or(String::from("div"))]
    pub component: String,
    /** Enables the body Content to fill the height of the card */
    #[prop_or(true)]
    pub is_filled: bool,
}

impl Component for CardBody
{
    type Message = ();
    type Properties = CardBodyProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <@{ctx.props().component.to_string()}
                class={classes!(
                    "pf-v5-c-card__body",
                    if !ctx.props().is_filled {"pf-m-no-fill"} else {""},
                    ctx.props().classes.clone(),
                )}
                // {...props}
            >
                {ctx.props().children.clone()}
            </@>
        }
    }
}
