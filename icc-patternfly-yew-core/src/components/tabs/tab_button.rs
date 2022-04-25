use yew::{
    prelude::*,
};


pub struct TabButton;

#[derive(Clone, PartialEq, Properties)]
pub struct TabButtonProperties
{
    /** content rendered inside the Tab content area. */
    pub children: Children,
    /** additional classes added to the Tab */
    #[prop_or_default]
    pub class_name: String,
    /** URL associated with the Tab. A Tab with an href will render as an <a> instead of a <button>. A Tab inside a <Tabs component="nav"> should have an href. */
    #[prop_or_default]
    pub href: String,
    // /** child reference for case in which a TabContent section is defined outside of a Tabs component */
    // tabContentRef?: React.Ref<any>;
    pub id: String,
    /** uniquely identifies the tab */
    #[prop_or_default]
    pub event_key:  String, // number | string;
    #[prop_or_default]
    pub onclick: Callback<String>,
}

pub enum TabButtonMsg
{
    OnClick,
}

impl Component for TabButton
{
    type Message = TabButtonMsg;
    type Properties = TabButtonProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            TabButtonMsg::OnClick => {
                ctx.props().onclick.emit(ctx.props().event_key.clone());

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <button
                class={ctx.props().class_name.to_string()}
                onclick={ctx.link().callback(|_| TabButtonMsg::OnClick)}
            >
                { ctx.props().children.clone() }
            </button>
        }
    }
}