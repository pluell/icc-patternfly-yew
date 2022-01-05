use yew::{
    prelude::*,
};

use crate::{Button, ButtonVariant};


pub struct FormFieldGroupToggle
{
    props: FormFieldGroupToggleProps,
}

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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
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
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        let ontoggle = self.props.ontoggle.clone();

        html!{
            <div 
                class=classes!(
                    "pf-c-form__field-group-toggle",
                    self.props.class_name.clone(),
                )
                // {...props}
            >
                <div class="pf-c-form__field-group-toggle-button">
                    <Button
                        variant=ButtonVariant::Plain
                        aria_label=self.props.aria_label.clone()
                        onclick=Callback::from(move |_| ontoggle.emit(()))
                        aria_expanded=self.props.is_expanded.to_string()
                        aria_labelledby=self.props.aria_labelledby.clone()
                        id=self.props.toggle_id.clone()
                    >
                        <span class="pf-c-form__field-group-toggle-icon">
                            {icc_patternfly_yew_icons::angle_right_icon!{}}
                        </span>
                    </Button>
                </div>
            </div>
        }
    }
}
