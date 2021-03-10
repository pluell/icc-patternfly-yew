use yew::{
    prelude::*,
};

use super::*;


pub struct Toolbar
{
    props: ToolbarProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarProperties
{
    pub children: Children,
    #[prop_or_default]
    pub clear_all_filters: Callback<()>,
    #[prop_or_default]
    pub clear_filters_button_text: String,
}

impl Component for Toolbar
{
    type Message = ();
    type Properties = ToolbarProperties;

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
        true
    }

    fn view(&self) -> Html
    {
        // TODO: Implement calculating number of filters
        let number_of_filters = 0;

        html!{
            <div class="pf-c-toolbar">
                { self.props.children.clone() }
                <ToolbarChipGroupContent 
                    show_clear_filters_button=(number_of_filters > 0)
                    clear_filters_button_text=self.props.clear_filters_button_text.clone()
                    clear_all_filters=self.props.clear_all_filters.clone()
                />
            </div>
        }
    }
}
