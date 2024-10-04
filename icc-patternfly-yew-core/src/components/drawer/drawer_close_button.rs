use yew::prelude::*;

use icc_patternfly_yew_icons::times_icon;

use crate::{Button, ButtonVariant};


pub struct DrawerCloseButton;

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerCloseButtonProps
{
    /** Additional classes added to the drawer close button outer <div>. */
    #[prop_or_default]
    pub classes: Classes,
    /** A callback for when the close button is clicked  */
    #[prop_or_default]
    pub onclose: Callback<()>,
    /** Accessible label for the drawer close button */
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
}

impl Component for DrawerCloseButton
{
    type Message = ();
    type Properties = DrawerCloseButtonProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let onclose = ctx.props().onclose.clone();

        html!{
            <div 
                class={classes!("pf-v5-c-drawer__close", ctx.props().classes.clone())}
                // {...props}
            >
                <Button 
                    variant={ButtonVariant::Plain}
                    onclick={Callback::from(move |_| onclose.emit(()))}
                    aria_label={ctx.props().aria_label.clone()}
                >
                    {times_icon!{}}
                </Button>
            </div>
        }
    }
}
