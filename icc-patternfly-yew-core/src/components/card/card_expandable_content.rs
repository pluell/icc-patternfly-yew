use yew::prelude::*;

use super::CardContext;

pub struct CardExpandableContent
{
    context: CardContext,
    _context_listener: ContextHandle<CardContext>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardExpandableContentProperties
{
    /** Content rendered inside the Card Body */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the Card Body */
    #[prop_or_default]
    pub classes: Classes,
}

pub enum CardExpandableContentMsg
{
    Context(CardContext),
}

impl Component for CardExpandableContent
{
    type Message = CardExpandableContentMsg;
    type Properties = CardExpandableContentProperties;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (context, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::Context))
            .expect("No Message Context Provided");

        Self {
            context,
            _context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool
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
            if self.context.is_expanded {
                <div
                    class={classes!(
                        "pf-v5-c-card__expandable-content", 
                        ctx.props().classes.clone()
                    )}
                    // {...props}
                >
                    {ctx.props().children.clone()}
                </div>
            }
        }
    }
}
