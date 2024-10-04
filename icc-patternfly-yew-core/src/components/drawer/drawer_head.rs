use yew::prelude::*;

use super::DrawerPanelBody;

pub struct DrawerHead;

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerHeadProps
{
    /** Additional classes added to the drawer head. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content to be rendered in the drawer head */
    #[prop_or_default]
    pub children: Html,
    /** Indicates if there should be no padding around the drawer panel body of the head*/
    #[prop_or_default]
    pub has_no_padding: bool,
}

impl Component for DrawerHead
{
    type Message = ();
    type Properties = DrawerHeadProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <DrawerPanelBody has_no_padding={ctx.props().has_no_padding}>
                <div 
                    class={classes!("pf-v5-c-drawer__head", ctx.props().classes.clone())}
                    // {...props}
                >
                    {ctx.props().children.clone()}
                </div>
            </DrawerPanelBody>
        }
    }
}
