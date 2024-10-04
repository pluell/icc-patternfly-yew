use yew::prelude::*;

use super::DrawerColorVariant;

pub struct DrawerSection;

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerSectionProps
{
    /** Additional classes added to the drawer section. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content to be rendered in the drawer section. */
    #[prop_or_default]
    pub children: Html,
    /** Color variant of the background of the drawer Section */
    #[prop_or(DrawerColorVariant::Default)]
    pub color_variant: DrawerColorVariant,
}

impl Component for DrawerSection
{
    type Message = ();
    type Properties = DrawerSectionProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-drawer__section",
                    ctx.props().color_variant.get_class(),
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
