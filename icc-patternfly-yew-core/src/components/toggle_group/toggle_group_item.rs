use yew::{
    prelude::*,
};

use super::*;


pub struct ToggleGroupItem;

#[derive(Clone, PartialEq, Properties)]
pub struct ToggleGroupItemProps
{
    /** Text rendered inside the toggle group item */
    #[prop_or_default]
    pub text: Option<Html>,
    /** Icon rendered inside the toggle group item */
    #[prop_or_default]
    pub icon: Option<Html>,
    /** Additional classes added to the toggle group item */
    #[prop_or_default]
    pub class_name: String,
    /** Flag indicating if the toggle group item is disabled */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag indicating if the toggle group item is selected */
    #[prop_or_default]
    pub is_selected: bool,
    /** required when icon is used with no supporting text */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Optional id for the button within the toggle group item */
    #[prop_or_default]
    pub button_id: String,
    /** A callback for when the toggle group item selection changes. */
    #[prop_or_default]
    pub onchange: Callback<bool>, // (selected: boolean, event: React.MouseEvent<any> | React.KeyboardEvent | MouseEvent) => void;
}

pub enum ToggleGroupItemMsg
{
    OnClick(MouseEvent),
}

impl Component for ToggleGroupItem
{
    type Message = ToggleGroupItemMsg;
    type Properties = ToggleGroupItemProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            ToggleGroupItemMsg::OnClick(_event) => {
                ctx.props().onchange.emit(!ctx.props().is_selected);
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-c-toggle-group__item", 
                    ctx.props().class_name.clone()
                )}
                // {...props}
            >
                <button
                    type="button"
                    class={classes!(
                        "pf-c-toggle-group__button", 
                        if ctx.props().is_selected { "pf-m-selected" } else { "" },
                    )}
                    aria-pressed={ctx.props().is_selected.to_string()}
                    onclick={ctx.link().callback(ToggleGroupItemMsg::OnClick)}
                    aria-label={ctx.props().aria_label.clone()}
                    disabled={ctx.props().is_disabled}
                    id={ctx.props().button_id.clone()}
                >
                {
                    if let Some(icon) = &ctx.props().icon
                    {
                        html!{
                            <ToggleGroupItemElement variant={ToggleGroupItemVariant::Icon}>{icon.clone()}</ToggleGroupItemElement>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if let Some(text) = &ctx.props().text
                    {
                        html!{
                            <ToggleGroupItemElement variant={ToggleGroupItemVariant::Text}>{text.clone()}</ToggleGroupItemElement>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                </button>
            </div>
        }
    }
}
