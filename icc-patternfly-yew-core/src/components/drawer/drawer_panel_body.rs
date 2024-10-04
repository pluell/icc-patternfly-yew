use yew::prelude::*;


pub struct DrawerPanelBody;

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerPanelBodyProps
{
    /** Additional classes added to the Drawer. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content to be rendered in the drawer */
    #[prop_or_default]
    pub children: Html,
    /** Indicates if there should be no padding around the drawer panel body */
    #[prop_or_default]
    pub has_no_padding: bool,
}

impl Component for DrawerPanelBody
{
    type Message = ();
    type Properties = DrawerPanelBodyProps;

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
                    if ctx.props().has_no_padding {"pf-m-no-padding"} else {""},
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
