use yew::prelude::*;


pub struct CardHeaderMain;

#[derive(Clone, PartialEq, Properties)]
pub struct CardHeaderMainProperties
{
    /** Content rendered inside the Card Head Main */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the Card Head Main */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for CardHeaderMain
{
    type Message = ();
    type Properties = CardHeaderMainProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-card__header-main",
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
            {
                ctx.props().children.clone()
            }
          </div>
        }
    }
}
