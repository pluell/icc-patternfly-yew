use yew::{
    prelude::*,
};

use crate::components::{Button, ButtonVariant};


pub struct ChipGroup
{
    _link: ComponentLink<Self>,
    props: ChipGroupProperties,
}

pub enum ChipGroupMsg
{
}

#[derive(Clone, PartialEq, Properties)]
pub struct ChipGroupProperties
{
    #[prop_or_default]
    pub default_is_open: bool,
    #[prop_or_default]
    pub category_name: String,
    #[prop_or_default]
    pub num_chips: i32,
    #[prop_or_default]
    pub is_closable: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

impl Component for ChipGroup
{
    type Message = ChipGroupMsg;
    type Properties = ChipGroupProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            _link: link,
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
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
        }
    }

    fn view(&self) -> Html
    {
        html!{
            <div
                class=classes!(
                    "pf-c-chip-group",
                    if self.props.category_name.is_empty() {""} else {"pf-m-category"},
                )
            >
                <div class="pf-c-chip-group__main">
                    { &self.props.category_name }
                    <ul class="pf-c-chip-group__list" role="list">
                    {
                        for self.props.children.iter().map(|child|
                            html!{
                                <li class="pf-c-chip-group__list-item">
                                {
                                    child.clone()
                                }
                                </li>
                            }
                        )
                    }
                    </ul>
                </div>
                {
                    if self.props.is_closable
                    {
                        html!{
                            <div class="pf-c-chip-group__close">
                                <Button
                                    variant=ButtonVariant::Plain
                                    onclick=self.props.onclick.clone()
                                >
                                    <i class="fas fa-times"></i>
                                </Button>
                            </div>
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
