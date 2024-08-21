use yew::prelude::*;

use crate::{Button, ButtonVariant};


pub struct CardHeader;

#[derive(Clone, PartialEq, Properties)]
pub struct CardHeaderProperties
{
    /** Content rendered inside the CardHeader */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the CardHeader */
    #[prop_or_default]
    pub class_name: String,
    /** ID of the card header. */
    #[prop_or_default]
    pub id: String,
    /** Callback expandable card */
    #[prop_or_default]
    pub onexpand: Option<Callback<String>>,
    /** Additional props for expandable toggle button */
    #[prop_or_default]
    pub toggle_button_props: String,
    #[prop_or_default]
    pub card_id: String,
}

pub enum CardHeaderMsg
{
    OnClickToggle,
}

impl Component for CardHeader
{
    type Message = CardHeaderMsg;
    type Properties = CardHeaderProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            CardHeaderMsg::OnClickToggle => {
                if let Some(onexpand) = &ctx.props().onexpand
                {
                    onexpand.emit(ctx.props().card_id.clone());
                }
                
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-card__header", 
                    ctx.props().class_name.clone(),
                )}
                id={ctx.props().id.clone()}
                // {...props}
            >
            {
                if let Some(_onexpand) = &ctx.props().onexpand
                {
                    html!{
                        <div class="pf-v5-c-card__header-toggle">
                            <Button
                                variant={ButtonVariant::Plain}
                                // type="button"
                                onclick={ctx.link().callback(|_| CardHeaderMsg::OnClickToggle)}
                                // {...toggleButtonProps}
                            >
                                <span class="pf-v5-c-card__header-toggle-icon">
                                    <i class="fas fa-angle-right" aria-hidden="true"></i>
                                </span>
                            </Button>
                        </div>
                    }
                }
                else
                {
                    html!{}
                }
            }
            {
                for ctx.props().children.iter()
            }
          </div>
        }
    }
}
