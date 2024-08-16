
use yew::prelude::*;


pub struct Badge;

#[derive(Clone, PartialEq, Properties)]
pub struct BadgeProperties
{
    /** Text announced by screen readers to indicate the current content/status of the badge. */
    #[prop_or_default]
    pub screen_reader_text: Option<String>,
    /**  Adds styling to the badge to indicate it has been read */
    #[prop_or_default]
    pub is_read: bool,
    /** content rendered inside the Badge */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the Badge */
    #[prop_or_default]
    pub classes: Option<Classes>,
}

impl Component for Badge
{
    type Message = ();
    type Properties = BadgeProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool
    {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html         
    {
        html!{
            <span
                // {...props}
                class={classes!(
                    "pf-v5-c-badge",
                    if ctx.props().is_read {"pf-m-read"} else {"pf-m-unread"},
                    ctx.props().classes.clone(),
                )}
            >
                {for ctx.props().children.iter()}
                {
                    if let Some(screen_reader_text) = &ctx.props().screen_reader_text
                    {
                        html!{<span class="pf-v5-screen-reader">{screen_reader_text}</span>}
                    }
                    else
                    {
                        html!{}
                    }
                }
            </span>
        }
    }
}