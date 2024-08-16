use yew::{prelude::*, virtual_dom::VTag};

use super::InputGroupItem;

#[derive(Clone, PartialEq)]
pub enum InputGroupTextVariant {
    Default,
    Plain,
}

pub struct InputGroupText;

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupTextProperties {
    /** Additional classes added to the input group text. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content rendered inside the input group text. */
    #[prop_or_default]
    pub children: Children,
    /** Component that wraps the input group text. */
    #[prop_or(String::from("span"))]
    pub component: String,
    /** Flag to to indicate if the input group item is plain. */
    #[prop_or_default]
    pub is_plain: bool,
    /** Flag to indicate if the input group text is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
}

impl Component for InputGroupText {
    type Message = ();
    type Properties = InputGroupTextProperties;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <InputGroupItem
                is_plain={ctx.props().is_plain} is_box={true} is_disabled={ctx.props().is_disabled}
            >
                {self.view_component(ctx)}
            </InputGroupItem>
        }
    }
}

impl InputGroupText
{
    fn view_component(&self, ctx: &Context<Self>) -> Html {
        let mut component = VTag::new(ctx.props().component.clone());

        component.add_attribute("class", classes!("pf-v5-c-input-group__text", ctx.props().classes.clone()).to_string());
        
        //     {...props}

        component.add_children(ctx.props().children.iter());

        component.into()
    }
}
