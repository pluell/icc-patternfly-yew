use yew::prelude::*;


pub struct DrawerMain;

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerMainProps
{
    /** Additional classes added to the drawer main wrapper. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content to be rendered in the drawer main wrapper*/
    #[prop_or_default]
    pub children: Html,
}

impl Component for DrawerMain
{
    type Message = ();
    type Properties = DrawerMainProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!("pf-v5-c-drawer__main", ctx.props().classes.clone())}
                // {...props}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
