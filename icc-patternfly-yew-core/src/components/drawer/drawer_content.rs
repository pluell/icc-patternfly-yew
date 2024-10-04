use yew::prelude::*;

use super::{DrawerContext, DrawerColorVariant, DrawerMain};

pub struct DrawerContent
{
    context: DrawerContext,
    _context_listener: ContextHandle<DrawerContext>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerContentProps
{
    /** Additional classes added to the Drawer. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content to be rendered in the drawer. */
    #[prop_or_default]
    pub children: Html,
    /** Content rendered in the drawer panel. */
    pub panel_content: Html,
    /** Color variant of the background of the drawer panel */
    #[prop_or(DrawerColorVariant::Default)]
    pub color_variant: DrawerColorVariant,
}

pub enum DrawerContentMessage
{
    Context(DrawerContext),
}

impl Component for DrawerContent
{
    type Message = DrawerContentMessage;
    type Properties = DrawerContentProps;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (context, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::Context))
            .expect("No Drawer Context Provided");

        Self {
            context,
            _context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg : Self::Message) -> bool
    {
        match msg
        {
            Self::Message::Context(context) => {
                self.context = context;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <DrawerMain>
                <div
                    class={classes!(
                        "pf-v5-c-drawer__content",
                        ctx.props().color_variant.get_class(),
                        ctx.props().classes.clone()
                    )}
                    ref={self.context.drawer_ref.clone()}
                    // {...props}
                >
                    {ctx.props().children.clone()}
                </div>
                {ctx.props().panel_content.clone()}
            </DrawerMain>
        }
    }
}
