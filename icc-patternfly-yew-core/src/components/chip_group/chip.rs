use yew::{
    prelude::*,
};

use crate::components::{Button, ButtonVariant};


pub struct Chip
{
    props: ChipProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ChipProperties
{
    pub children: Children,
    #[prop_or_default]
    pub is_read_only: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for Chip
{
    type Message = ();
    type Properties = ChipProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            // link,
            props,
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
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div
                class="pf-c-chip"
            >
                <span class="pf-c-chip__text">
                    {self.props.children.clone() }
                </span>
                {
                    if !self.props.is_read_only
                    {
                        html!{
                            <Button
                                variant=ButtonVariant::Plain
                                onclick=self.props.onclick.clone()
                            >
                                <i class="fas fa-times"></i>
                            </Button>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
            </div>
        }
    }
}
