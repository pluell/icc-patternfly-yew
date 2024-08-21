use yew::prelude::*;

use crate::{Button, ButtonVariant};


pub enum DataListToggleMsg
{
    OnClick,
}

pub struct DataListToggle;

#[derive(Clone, PartialEq, Properties)]
pub struct DataListToggleProps
{
    /** Additional classes added to the DataList cell */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show if the expanded content of the DataList item is visible */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Identify the DataList toggle number */
    pub id: String,
    /** Id for the row */
    #[prop_or_default]
    pub rowid: String,
    /** Adds accessible text to the DataList toggle */
    #[prop_or_default]
    pub aria_labelledby: String,
    /** Adds accessible text to the DataList toggle */
    #[prop_or_default]
    pub aria_label: String,
    /** Allows users of some screen readers to shift focus to the controlled element. Should be used when the controlled contents are not adjacent to the toggle that controls them. */
    #[prop_or_default]
    pub aria_controls: Option<String>,

    pub onclick: Callback<()>,
}

impl Component for DataListToggle
{
    type Message = DataListToggleMsg;
    type Properties = DataListToggleProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            DataListToggleMsg::OnClick => {
                ctx.props().onclick.emit(());
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-data-list__item-control",
                    ctx.props().class_name.clone()
                )}
                // {...props}
            >
                <div class="pf-v5-c-data-list__toggle">
                    <Button
                        id={ctx.props().id.clone()}
                        variant={ButtonVariant::Plain}
                        aria_controls={ctx.props().aria_controls.clone()}
                        aria_label={ctx.props().aria_label.to_string()}
                        aria_labelledby={if ctx.props().aria_label != "Details" { None } else { Some(format!("{}-{}", ctx.props().rowid, ctx.props().id)) }}
                        aria_expanded={ctx.props().is_expanded.to_string()}

                        onclick={ctx.link().callback(|_| DataListToggleMsg::OnClick)}
                    >
                        <div class={"pf-v5-c-data-list__toggle-icon"}>
                            {icc_patternfly_yew_icons::angle_right_icon!{}}
                        </div>
                    </Button>
                </div>
            </div>
        }
    }
}
