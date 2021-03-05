use yew::{
    prelude::*,
};

pub struct Switch
{
    link: ComponentLink<Self>,
    props: SwitchProperties,
}

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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            link,
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
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            SwitchMsg::OnClick => {
                self.props.onchange.emit(self.props.is_checked);

                false
            }
        }
    }

    fn view(&self) -> Html
    {
        html!{
            <label
                class=("pf-c-switch", self.props.class_name.to_string())
                for=self.props.id
                // {...getOUIAProps(Switch.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId, ouiaSafe)}
            >
                <input
                    id=self.props.id
                    class="pf-c-switch__input"
                    type="checkbox"
                    onclick=self.link.callback(|_| SwitchMsg::OnClick)
                    checked=self.props.is_checked
                    disabled=self.props.is_disabled
                    // aria-labelledby={isAriaLabelledBy ? `${this.id}-on` : null}
                    // {...props}
                />
                {
                    if !self.props.label.is_empty()
                    {
                        html!{
                            <>
                                <span class="pf-c-switch__toggle" />
                                <span
                                    class="pf-c-switch__label pf-m-on"
                                    // id={isAriaLabelledBy ? `${this.id}-on` : null}
                                    aria-hidden="true"
                                >
                                    {&self.props.label}
                                </span>
                                <span
                                    class="pf-c-switch__label pf-m-off"
                                    // id={isAriaLabelledBy ? `${this.id}-off` : null}
                                    aria-hidden="true"
                                >
                                    { if !self.props.label_off.is_empty() { &self.props.label_off } else { &self.props.label } }
                                </span>
                            </>
                        }
                    }
                    else
                    {
                        html!{
                            <span class="pf-c-switch__toggle">
                                <div class="pf-c-switch__toggle-icon" aria-hidden="true">
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
