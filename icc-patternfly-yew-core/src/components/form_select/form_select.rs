use web_sys::HtmlSelectElement;
use yew::prelude::*;

use icc_patternfly_yew_icons::caret_down_icon;

use crate::{FormSelectOption, ValidatedOptions};

use crate::components::{FormControlIcon, FromControlIconStatus};

pub struct FormSelect;

#[derive(Clone, PartialEq, Properties)]
pub struct FormSelectProperties
{
    /** content rendered inside the FormSelect */
    #[prop_or_default]
    pub children: ChildrenWithProps<FormSelectOption>,
    /** additional classes added to the FormSelect control */
    #[prop_or_default]
    pub classes: Classes,
    /** value of selected option */
    #[prop_or_default]
    pub value: Option<AttrValue>, //any;
    /** Value to indicate if the select is modified to show that validation state.
     * If set to success, select will be modified to indicate valid state.
     * If set to error, select will be modified to indicate error state.
     */
    #[prop_or(ValidatedOptions::Default)]
    pub validated: ValidatedOptions,
    /** Flag indicating the FormSelect is disabled */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Sets the FormSelect required. */
    #[prop_or_default]
    pub is_required: bool,
    /** Optional callback for updating when selection loses focus */
    #[prop_or_default]
    pub onblur: Callback<FocusEvent>, // (event: React.FormEvent<HTMLSelectElement>) => void;
    /** Optional callback for updating when selection gets focus */
    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>, // (event: React.FormEvent<HTMLSelectElement>) => void;
    /** Optional callback for updating when selection changes */
    #[prop_or_default]
    pub onchange: Callback<String>, // (event: React.FormEvent<HTMLSelectElement>, value: string) => void;
    /** Custom flag to show that the FormSelect requires an associated id or aria-label. */
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<AttrValue>, //number | string;
    /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    #[prop_or_default]
    pub ouia_safe: bool,

    #[prop_or_default]
    pub id: Option<AttrValue>,
}

pub enum FormSelectMsg
{
    OnChange(Event)
}

impl Component for FormSelect
{
    type Message = FormSelectMsg;
    type Properties = FormSelectProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            Self::Message::OnChange(event) => {
                if let Some(target_input) = event.target_dyn_into::<HtmlSelectElement>() {
                    ctx.props().onchange.emit(target_input.value())
                }

                false
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html
    {
        // Check if the selected option is a placeholder
        let is_placeholder_selected = ctx.props().children.iter()
            .filter(|child| child.props.value == ctx.props().value)
            .next()
            .map(|selected| selected.props.is_placeholder)
            .unwrap_or(false);

        let has_status_icon = ctx.props().validated != ValidatedOptions::Default;

        html!{
            <span
                class={classes!(
                    "pf-v5-c-form-control",
                    if ctx.props().is_disabled {"pf-m-disabled"} else {""},
                    if is_placeholder_selected {"pf-m-placeholder"} else {""},
                    if has_status_icon {format!("pf-m-{}", ctx.props().validated.to_string())} else {String::new()},
                    ctx.props().classes.clone(),
                )}
            >
                <select
                    // {...props}
                    id={ctx.props().id.clone()}
                    onblur={ctx.props().onblur.clone()}
                    onfocus={ctx.props().onfocus.clone()}
                    aria-label={ctx.props().aria_label.clone()}
                    aria-invalid={(ctx.props().validated == ValidatedOptions::Error).to_string()}
                    // {...getOUIAProps(FormSelect.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId, ouiaSafe)}
                    onchange={ctx.link().callback(Self::Message::OnChange)}
                    disabled={ctx.props().is_disabled}
                    required={ctx.props().is_required}
                    // value={&ctx.props().value}
                >
                    { 
                        for ctx.props().children.iter().map(|mut option| {
                            let props = std::rc::Rc::make_mut(&mut option.props);

                            // Mark the child as selected if the value matches
                            props.is_selected = props.value == ctx.props().value;

                            option
                        }) 
                    }
                </select>
                <span class="pf-v5-c-form-control__utilities">
                    {
                        html!{
                            if has_status_icon {
                                <FormControlIcon status={
                                    match ctx.props().validated {
                                        ValidatedOptions::Success => Some(FromControlIconStatus::Success),
                                        ValidatedOptions::Error => Some(FromControlIconStatus::Error),
                                        ValidatedOptions::Warning => Some(FromControlIconStatus::Warning),
                                        _ => None,
                                    }}
                                />
                            }
                        }
                    }
                    <span class="pf-v5-c-form-control__toggle-icon">
                        {caret_down_icon!{}}
                    </span>
                </span>
            </span>
        }
    }
}
