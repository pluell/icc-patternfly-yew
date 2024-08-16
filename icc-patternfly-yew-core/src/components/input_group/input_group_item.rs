use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum InputGroupItemVariant {
    Default,
    Plain,
}

pub struct InputGroupItem;

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupItemProperties {
    /** Additional classes added to the input group item. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content rendered inside the input group item. */
    #[prop_or_default]
    pub children: Children,
    /** Enables box styling to the input group item */
    #[prop_or_default]
    pub is_box: bool,
    /** Flag to indicate if the input group item is plain. */
    #[prop_or_default]
    pub is_plain: bool,
    /** Flag to indicate if the input group item should fill the available horizontal space */
    #[prop_or_default]
    pub is_fill: bool,
    /** Flag to indicate if the input group item is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
}

impl Component for InputGroupItem {
    type Message = ();
    type Properties = InputGroupItemProperties;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
                class={classes!(
                    "pf-v5-c-input-group__item",
                    if ctx.props().is_fill {"pf-m-fill"} else {""},
                    if ctx.props().is_box {"pf-m-box"} else {""},
                    if ctx.props().is_plain {"pf-m-plain"} else {""},
                    if ctx.props().is_disabled {"pf-m-disabled"} else {""},
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}
