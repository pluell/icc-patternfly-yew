use yew::prelude::*;

use crate::{Button, ButtonVariant};


pub struct FormFieldGroupToggle;

#[derive(Clone, PartialEq, Properties)]
pub struct FormFieldGroupToggleProps
{
    /** Additional classes added to the section */
    #[prop_or_default]
    pub class_name: String,
    /** Callback for onClick */
    #[prop_or_default]
    pub ontoggle: Callback<()>,
    /** Flag indicating if the toggle is expanded */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Aria label of the toggle button */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Sets the aria-labelledby attribute on the toggle button element */
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
    /** The id applied to the toggle button */
    #[prop_or_default]
    pub toggle_id: Option<String>,
}

impl Component for FormFieldGroupToggle
{
    type Message = ();
    type Properties = FormFieldGroupToggleProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let ontoggle = ctx.props().ontoggle.clone();

        html!{
            <div 
                class={classes!(
                    "pf-v5-c-form__field-group-toggle",
                    ctx.props().class_name.clone(),
                )}
                // {...props}
            >
                <div class="pf-v5-c-form__field-group-toggle-button">
                    <Button
                        variant={ButtonVariant::Plain}
                        aria_label={ctx.props().aria_label.clone()}
                        onclick={Callback::from(move |_| ontoggle.emit(()))}
                        aria_expanded={ctx.props().is_expanded.to_string()}
                        aria_labelledby={ctx.props().aria_labelledby.clone()}
                        id={ctx.props().toggle_id.clone()}
                    >
                        <span class="pf-v5-c-form__field-group-toggle-icon">
                            {icc_patternfly_yew_icons::angle_right_icon!{}}
                        </span>
                    </Button>
                </div>
            </div>
        }
    }
}
