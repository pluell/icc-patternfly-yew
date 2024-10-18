use yew::prelude::*;


pub struct CardActions;

#[derive(Clone, PartialEq, Properties)]
pub struct CardActionsProperties
{
    /** Content rendered inside the Card Action */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the Action */
    #[prop_or_default]
    pub classes: Classes,
    /** Flag indicating that the actions have no offset */
    #[prop_or_default]
    pub has_no_offset: bool,
}

impl Component for CardActions
{
    type Message = ();
    type Properties = CardActionsProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-card__actions",
                    if ctx.props().has_no_offset {"pf-m-no-offset"} else {""},
                    ctx.props().classes.clone()
                )}
            >
            {
                ctx.props().children.clone()
            }
          </div>
        }
    }
}
