use yew::{
    prelude::*,
};

use crate::{Button, ButtonVariant};


pub struct CardHeader
{
    props: CardHeaderProperties,
    link: ComponentLink<CardHeader>,
}

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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            props,
            link,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            
            true
        }
        else
        {
            false
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            CardHeaderMsg::OnClickToggle => {
                if let Some(onexpand) = &self.props.onexpand
                {
                    onexpand.emit(self.props.card_id.clone());
                }
                
            },
        }

        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div 
                class=(
                    "pf-c-card__header", 
                    self.props.class_name.clone(),
                )
                id=&self.props.id
                // {...props}
            >
            {
                if let Some(onexpand) = &self.props.onexpand
                {
                    html!{
                        <div class="pf-c-card__header-toggle">
                            <Button
                                variant=ButtonVariant::Plain
                                // type="button"
                                onclick=self.link.callback(|_| CardHeaderMsg::OnClickToggle)
                                // {...toggleButtonProps}
                            >
                                <span class="pf-c-card__header-toggle-icon">
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
                for self.props.children.iter()
            }
          </div>
        }
    }
}
