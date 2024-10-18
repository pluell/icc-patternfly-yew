use web_sys::HtmlInputElement;
use yew::prelude::*;


pub struct Radio;

#[derive(Clone, PartialEq, Properties)]
pub struct RadioProps
{
    /** Additional classes added to the radio wrapper. This wrapper will be div element by default. It will be a label element if
     * isLabelWrapped is true, or it can be overridden by any element specified in the component prop.
     */
    #[prop_or_default]
    pub classes: Classes,
    /** Additional classes added to the radio input. */
    #[prop_or_default]
    pub input_classes: Classes,
    /** Id of the radio. */
    pub id: AttrValue,
    /** Flag to indicate whether the radio wrapper element is a native label element for the radio input. Will not apply if a component prop (with a value other than a "label") is specified. */
    #[prop_or_default]
    pub is_label_wrapped: bool,
    /** Flag to show if the radio label is shown before the radio input. */
    #[prop_or_default]
    pub is_label_before_button: bool,
    /** Flag to show if the radio is checked. */
    #[prop_or_default]
    pub checked: bool,
    /** Flag to show if the radio is checked. */
    // #[prop_or(Some(false))]
    #[prop_or_default]
    pub is_checked: Option<bool>,
    /** Flag to show if the radio is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag to show if the radio selection is valid or invalid. */
    #[prop_or_default]
    pub is_valid: bool,
    /** Label text of the radio. */
    #[prop_or_default]
    pub label: Option<Html>,
    /** Name for group of radios */
    pub name: AttrValue,
    /** A callback for when the radio selection changes. */
    #[prop_or_default]
    pub onchange: Callback<bool>,
    /** Aria label for the radio. */
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /** Description text of the radio. */
    #[prop_or_default]
    pub description: Option<Html>,
    /** Body of the radio. */
    #[prop_or_default]
    pub body: Option<Html>,
    /** Sets the radio wrapper component to render. Defaults to "div". If set to "label", behaves the same as if isLabelWrapped prop was specified. */
    #[prop_or_default]
    pub component: Option<String>,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<AttrValue>,
    /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    #[prop_or(true)]
    pub ouia_safe: bool,

    // Props
    #[prop_or_default]
    pub default_checked: Option<bool>,
}

pub enum RadioMsg
{
    OnChange(Event)
}

impl Component for Radio
{
    type Message = RadioMsg;
    type Properties = RadioProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            Self::Message::OnChange(event) => {
                if let Some(target_input) = event.target_dyn_into::<HtmlInputElement>() {
                    ctx.props().onchange.emit(target_input.checked())
                }

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let wrap_with_label = (ctx.props().is_label_wrapped && !ctx.props().component.is_none()) || 
        ctx.props().component == Some(String::from("label"));

        let component = if let Some(component) = &ctx.props().component {
            component.clone()
        } else {
            if wrap_with_label {
                String::from("label")
            } else {
                String::from("div")
            }
        };

        html!{
            <@{component}
                class={classes!(
                    "pf-v5-c-radio",
                    if ctx.props().label.is_none() {"pf-m-standalone"} else {""},
                    ctx.props().classes.clone()
                )}
                for={if wrap_with_label {Some(ctx.props().id.clone())} else {None}}
            >
            {
                if ctx.props().is_label_before_button {
                    html!{
                        <>
                            {self.view_label(ctx, wrap_with_label)}
                            {self.view_input(ctx)}
                        </>
                    }
                } else {
                    html!{
                        <>
                            {self.view_input(ctx)}
                            {self.view_label(ctx, wrap_with_label)}
                        </>
                    }
                }
            }
            {
                html!{
                    if let Some(description) = &ctx.props().description {
                        <span class="pf-v5-c-radio__description">{description.clone()}</span>
                    }
                }
            }
            {
                html!{
                    if let Some(body) = &ctx.props().body {
                        <span class="pf-v5-c-radio__body">{body.clone()}</span>
                    }
                }
            }
          </@>
        }
    }
}

impl Radio
{
    fn view_label(&self, ctx: &Context<Self>, wrap_with_label: bool) -> Html
    {
        let label_component = if wrap_with_label {
            String::from("span")
        } else {
            String::from("label")
        };

        html!{
            if let Some(label) = &ctx.props().label {
                <@{label_component}
                    class={classes!(
                        "pf-v5-c-radio__label", 
                        if ctx.props().is_disabled {"pf-m-disabled"} else {""},
                    )}
                    for={if !wrap_with_label {Some(ctx.props().id.clone())} else {None}}
                >
                {
                    label.clone()
                }
                </@>
            }
        }
    }

    fn view_input(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <input
                // {...props}
                id={ctx.props().id.clone()}
                name={ctx.props().name.clone()}
                class={classes!(
                    "pf-v5-c-radio__input",
                    ctx.props().input_classes.clone()
                )}
                type="radio"
                onchange={ctx.link().callback(RadioMsg::OnChange)}
                aria-invalid={(!ctx.props().is_valid).to_string()}
                disabled={ctx.props().is_disabled}
                checked={ctx.props().is_checked.unwrap_or(ctx.props().default_checked.unwrap_or(false))}
                aria-label={if ctx.props().label.is_none() {ctx.props().aria_label.clone()} else {None}}
                // {...getOUIAProps(Radio.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId, ouiaSafe)}
                ouia-id={ctx.props().ouia_id.clone()}
                data-ouia-safe={ctx.props().ouia_safe.to_string()}
            />
        }
        
    }
}