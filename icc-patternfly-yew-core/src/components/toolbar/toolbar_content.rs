use yew::{
    prelude::*,
};


pub struct ToolbarContent
{
    props: ToolbarContentProperties,
}

pub enum ToolbarContentMsg
{
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarContentProperties
{
    pub children: Children,
}

impl Component for ToolbarContent
{
    type Message = ToolbarContentMsg;
    type Properties = ToolbarContentProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
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
            <div class="pf-c-toolbar__content">
                <div class="pf-c-toolbar__content-section">
                    { self.props.children.clone() }
                </div>
            </div>
        }
    }
}
