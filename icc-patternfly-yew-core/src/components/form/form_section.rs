use yew::{
    prelude::*,
};

pub struct FormSection
{
    props: FormSectionProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormSectionProperties
{
    /** Content rendered inside the section */
    pub children: Children,
    /** Additional classes added to the FormSection. */
    #[prop_or_default]
    pub class_name: String,
    /** Label text for the section. */
    #[prop_or_default]
    pub label: String,
}

impl Component for FormSection
{
    type Message = ();
    type Properties = FormSectionProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            // link,
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
        html!{
            <section
                // {...props} 
                class=classes!("pf-c-form__section", self.props.class_name.to_string())
            >
                {
                    if !self.props.label.is_empty()
                    {
                        html!{
                            <div class="pf-c-form__section-title">{&self.props.label}</div>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                { self.props.children.clone() }
            </section>
        }
    }
}
