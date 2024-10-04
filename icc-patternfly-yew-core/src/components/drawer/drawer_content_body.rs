use yew::prelude::*;


pub struct DrawerContentBody;

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerContentBodyProps
{
    /** Additional classes added to the Drawer. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content to be rendered in the drawer */
    #[prop_or_default]
    pub children: Html,
    /** Indicates if there should be padding around the drawer content body */
    #[prop_or_default]
    pub has_padding: bool,
}

impl Component for DrawerContentBody
{
    type Message = ();
    type Properties = DrawerContentBodyProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-drawer__body", 
                    if ctx.props().has_padding {"pf-m-padding"} else {""},
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
