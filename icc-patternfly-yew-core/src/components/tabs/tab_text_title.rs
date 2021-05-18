use yew::{
    prelude::*,
};


pub struct TabTextTitle
{
    props: TabTextTitleProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabTextTitleProperties
{
    /** Text to be rendered inside the tab button title. */
    pub children: Children,
    /** additional classes added to the tab title text */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for TabTextTitle
{
    type Message = ();
    type Properties = TabTextTitleProperties;

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
            <span class=classes!("pf-c-tabs__item-text", self.props.class_name.to_string())>
                { self.props.children.clone() }
            </span>
        }
    }
}