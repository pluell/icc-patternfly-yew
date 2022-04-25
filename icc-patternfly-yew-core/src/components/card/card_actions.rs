use yew::{
    prelude::*,
};


pub struct CardActions;

#[derive(Clone, PartialEq, Properties)]
pub struct CardActionsProperties
{
    /** Content rendered inside the Card Action */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Action */
    #[prop_or_default]
    pub class_name: String,
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
                    "pf-c-card__actions",
                    ctx.props().class_name.clone()
                )}
            >
            {
                for ctx.props().children.iter()
            }
          </div>
        }
    }
}
