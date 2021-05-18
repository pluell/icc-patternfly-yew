use yew::{
    prelude::*,
};

pub struct FormAlert
{
    props: FormAlertProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormAlertProperties
{
    /** An inline PatternFly Alert. */
    pub children: Children,
    /** Additional classes added to the FormAlert. */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for FormAlert
{
    type Message = ();
    type Properties = FormAlertProperties;

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
            <div 
                // {...props} 
                class=classes!("pf-c-form__alert", self.props.class_name.to_string())
            >
            { self.props.children.clone() }
            </div>
        }
    }
}
