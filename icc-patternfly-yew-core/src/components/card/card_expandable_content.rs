use yew::prelude::*;


pub struct CardExpandableContent;

#[derive(Clone, PartialEq, Properties)]
pub struct CardExpandableContentProperties
{
    /** Content rendered inside the Card Body */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Card Body */
    #[prop_or_default]
    pub class_name: String,
    /** Flag indicating if a card is expanded. Modifies the card to be expandable. */
    #[prop_or_default]
    pub is_expanded: bool,
}

impl Component for CardExpandableContent
{
    type Message = ();
    type Properties = CardExpandableContentProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        if ctx.props().is_expanded
        {
            html!{
                <div
                    class={classes!(
                        "pf-v5-c-card__expandable-content", 
                        ctx.props().class_name.clone()
                    )}
                    // {...props}
                >
                    { for ctx.props().children.iter() }
                </div>
            }
        }
        else
        {
            html!{}
        }
    }
}
