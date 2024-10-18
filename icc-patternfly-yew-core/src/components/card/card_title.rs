use yew::prelude::*;

use super::CardContext;


pub struct CardTitle
{
    context: CardContext,
    _context_listener: ContextHandle<CardContext>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardTitleProperties
{
    /** Content rendered inside the CardTitle */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the CardTitle */
    #[prop_or_default]
    pub classes: String,
    /** Sets the base component to render. defaults to div */
    #[prop_or(String::from("div"))]
    pub component: String,
}

pub enum CardTitleMsg
{
    Context(CardContext),
}

impl Component for CardTitle
{
    type Message = CardTitleMsg;
    type Properties = CardTitleProperties;

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
        let title_id = if self.context.card_id.len() > 0 {
            Some(format!("{}-title", self.context.card_id))
        } else {
            None
        };

        html!{
            <@{ctx.props().component.to_string()}
                class={classes!(
                    "pf-v5-c-card__title", 
                    ctx.props().classes.clone()
                )}
                id={title_id}
                // {...props}
            >
                {ctx.props().children.clone()}
            </@>
        }
    }
}
