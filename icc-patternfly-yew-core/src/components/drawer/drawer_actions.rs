use yew::prelude::*;


pub struct DrawerActions;

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerActionsProps
{
    /** Additional classes added to the drawer actions button. */
    #[prop_or_default]
    pub classes: Classes,
    /** Actions to be rendered in the panel head. */
    #[prop_or_default]
    pub children: Html,
}

impl Component for DrawerActions
{
    type Message = ();
    type Properties = DrawerActionsProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!("pf-v5-c-drawer__actions", ctx.props().classes.clone())}
                // {...props}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
