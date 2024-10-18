use yew::prelude::*;


pub struct CardSelectableActions;

#[derive(Clone, PartialEq, Properties)]
pub struct CardSelectableActionsProps
{
    /** Content rendered inside the card action */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the action */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for CardSelectableActions
{
    type Message = ();
    type Properties = CardSelectableActionsProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-card__selectable-actions",
                    ctx.props().classes.clone()
                )}
            >
                {ctx.props().children.clone()}
          </div>
        }
    }
}
