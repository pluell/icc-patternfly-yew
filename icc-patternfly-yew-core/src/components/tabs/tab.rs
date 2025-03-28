use yew::prelude::*;

pub struct Tab;

#[derive(Clone, PartialEq, Properties)]
pub struct TabProperties
{
    /** content rendered inside the Tab content area. */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the Tab */
    #[prop_or_default]
    pub class_name: String,
    /** URL associated with the Tab. A Tab with an href will render as an <a> instead of a <button>. A Tab inside a <Tabs component="nav"> should have an href. */
    #[prop_or_default]
    pub href: String,
    /** Content rendered in the tab title. Should be <TabTitleText> and/or <TabTitleIcon> for proper styling. */
    #[prop_or_default]
    pub title: Html,
    /** uniquely identifies the tab */
    #[prop_or_default]
    pub event_key:  String,
    /** child id for case in which a TabContent section is defined outside of a Tabs component */
    #[prop_or_default]
    pub tab_content_id: String,
    /** whether to render the tab or not */
    #[prop_or_default]
    pub is_hidden: bool,
}

impl Component for Tab
{
    type Message = ();
    type Properties = TabProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html
    {
        html!{}
    }
}
