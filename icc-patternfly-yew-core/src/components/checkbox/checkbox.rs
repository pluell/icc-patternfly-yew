use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::ASTERISK;


pub struct Checkbox
{
    input_ref: NodeRef,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CheckboxProps
{
    /** Additional classes added to the checkbox wrapper. This wrapper will be div element by default. It will be a label element if
     * isLabelWrapped is true, or it can be overridden by any element specified in the component prop.
     */
    #[prop_or_default]
    pub classes: Classes,
    /** Additional classes added to the checkbox input. */
    #[prop_or_default]
    pub input_classes: Classes,
    /** Flag to indicate whether the checkbox wrapper element is a <label> element for the checkbox input. Will not apply if a component prop (with a value other than a "label") is specified. */
    #[prop_or_default]
    pub is_label_wrapped: bool,
    /** Flag to show if the checkbox label is shown before the checkbox input. */
    #[prop_or_default]
    pub is_label_before_button: bool,
    /** Flag to show if the checkbox selection is valid or invalid. */
    #[prop_or_default]
    pub is_valid: bool,
    /** Flag to show if the checkbox is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag to show if the checkbox is required. */
    #[prop_or_default]
    pub is_required: bool,
    /** Flag to show if the checkbox is checked. If null, the checkbox will be indeterminate (partially checked). */
    // #[prop_or(Some(false))]
    #[prop_or_default]
    // #[prop_or(Some(Some(false)))]
    pub is_checked: Option<Option<bool>>,
    #[prop_or_default]
    pub checked: bool,
    /** A callback for when the checkbox selection changes. */
    #[prop_or_default]
    pub onchange: Callback<bool>, // (event: React.FormEvent<HTMLInputElement>, checked: boolean) => void;
    /** Label text of the checkbox. */
    #[prop_or_default]
    pub label: Option<Html>,
    /** Id of the checkbox. */
    pub id: AttrValue,
    /** Aria-label of the checkbox. */
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /** Description text of the checkbox. */
    #[prop_or_default]
    pub description: Option<Html>,
    /** Body text of the checkbox */
    #[prop_or_default]
    pub body: Option<Html>,
    /** Sets the checkbox wrapper component to render. Defaults to "div". If set to "label", behaves the same as if isLabelWrapped prop was specified. */
    // #[prop_or(String::from("div"))]
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
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub default_checked: Option<bool>,
}

pub enum CheckboxMsg
{
    OnChange(Event)
}

impl Component for Checkbox
{
    type Message = CheckboxMsg;
    type Properties = CheckboxProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self {
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            CheckboxMsg::OnChange(event) => {
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
                    "pf-v5-c-check",
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
                        <span class="pf-v5-c-check__description">{description.clone()}</span>
                    }
                }
            }
            {
                html!{
                    if let Some(body) = &ctx.props().body {
                        <span class="pf-v5-c-check__body">{body.clone()}</span>
                    }
                }
            }
          </@>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool)
    {
        if let Some(input_ref) = self.input_ref.cast::<HtmlInputElement>()
        {
            if let Some(is_checked) = ctx.props().is_checked {
                input_ref.set_indeterminate(is_checked.is_none()); 
            } else {
                input_ref.set_indeterminate(false);
            }

            // Set the defaultChecked property
            if let Some(default_checked) = ctx.props().default_checked {
                input_ref.set_default_checked(default_checked);
            }
        }
    }
}

impl Checkbox
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
                        "pf-v5-c-check__label", 
                        if ctx.props().is_disabled {"pf-m-disabled"} else {""},
                    )}
                    for={if !wrap_with_label {Some(ctx.props().id.clone())} else {None}}
                >
                {
                    label.clone()
                }
                {
                    html!{
                        if ctx.props().is_required {
                            <span class="pf-v5-c-check__label-required" aria-hidden="true">
                                {ASTERISK}
                            </span>
                        }
                    }
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
                    "pf-v5-c-check__input",
                    ctx.props().input_classes.clone()
                )}
                type="checkbox"
                onchange={ctx.link().callback(CheckboxMsg::OnChange)}
                aria-invalid={(!ctx.props().is_valid).to_string()}
                aria-label={ctx.props().aria_label.clone()}
                disabled={ctx.props().is_disabled}
                required={ctx.props().is_required}
                ref={self.input_ref.clone()}
                // {...checkedProps}
                checked={ctx.props().is_checked.unwrap_or(ctx.props().default_checked.clone()).unwrap_or(false)}
                // {...getOUIAProps(Checkbox.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId, ouiaSafe)}
                data-ouia-component-id={ctx.props().ouia_id.clone()}
                data-ouia-safe={ctx.props().ouia_safe.to_string()}
            />
        }
        
    }
}