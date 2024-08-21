use yew::prelude::*;

pub struct Switch;

#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProperties
{
    /** id for the label. */
    pub id: String,
    /** Additional classes added to the Switch */
    #[prop_or_default]
    pub class_name: String,
    /** Text value for the label when on */
    #[prop_or_default]
    pub label: String, //React.ReactNode;
    /** Text value for the label when off */
    #[prop_or_default]
    pub label_off: String, //React.ReactNode;
    /** Flag to show if the Switch is checked. */
    #[prop_or_default]
    pub is_checked: bool,
    /** Flag to show if the Switch is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    /** A callback for when the Switch selection changes. (isChecked, event) => {} */
    #[prop_or_default]
    pub onchange: Callback<bool>,
    // /** Adds accessible text to the Switch, and should describe the isChecked="true" state. When label is defined, aria-label should be set to the text string that is visible when isChecked is true. */
    // 'aria-label'?: string;
}

pub enum SwitchMsg
{
    OnClick,
}

impl Component for Switch
{
    type Message = SwitchMsg;
    type Properties = SwitchProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            SwitchMsg::OnClick => {
                ctx.props().onchange.emit(ctx.props().is_checked);

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <label
                class={classes!("pf-v5-c-switch", ctx.props().class_name.to_string())}
                for={ctx.props().id.clone()}
                // {...getOUIAProps(Switch.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId, ouiaSafe)}
            >
                <input
                    id={ctx.props().id.clone()}
                    class="pf-v5-c-switch__input"
                    type="checkbox"
                    onclick={ctx.link().callback(|_| SwitchMsg::OnClick)}
                    checked={ctx.props().is_checked}
                    disabled={ctx.props().is_disabled}
                    // aria-labelledby={isAriaLabelledBy ? `${this.id}-on` : null}
                    // {...props}
                />
                {
                    if !ctx.props().label.is_empty()
                    {
                        html!{
                            <>
                                <span class="pf-v5-c-switch__toggle" />
                                <span
                                    class="pf-v5-c-switch__label pf-m-on"
                                    // id={isAriaLabelledBy ? `${this.id}-on` : null}
                                    aria-hidden="true"
                                >
                                    {&ctx.props().label}
                                </span>
                                <span
                                    class="pf-v5-c-switch__label pf-m-off"
                                    // id={isAriaLabelledBy ? `${this.id}-off` : null}
                                    aria-hidden="true"
                                >
                                    { if !ctx.props().label_off.is_empty() { &ctx.props().label_off } else { &ctx.props().label } }
                                </span>
                            </>
                        }
                    }
                    else
                    {
                        html!{
                            <span class="pf-v5-c-switch__toggle">
                                <div class="pf-v5-c-switch__toggle-icon" aria-hidden="true">
                                    // <CheckIcon noVerticalAlign />
                                </div>
                            </span>
                        }
                    }
                }
            </label>
        }
    }
}
