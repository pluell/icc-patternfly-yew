use yew::{
    prelude::*,
};

pub struct Form
{
    // link: ComponentLink<Self>,
    props: FormProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormProperties
{
    /** Anything that can be rendered as Form content. */
    pub children: Children,
    /** Additional classes added to the Form. */
    #[prop_or_default]
    pub class_name: String,
    /** Sets the Form to horizontal. */
    #[prop_or_default]
    pub is_horizontal: bool,
    /** Flag to limit the max-width to 500px. */
    #[prop_or_default]
    pub is_width_limited: bool,
}

impl Component for Form
{
    type Message = ();
    type Properties = FormProperties;

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
            <form
                no_validate=true
                class=(
                    "pf-c-form",
                    if self.props.is_horizontal {"pf-m-horizontal"} else {""},
                    if self.props.is_width_limited {"pf-m-limit-width"} else {""},
                    self.props.class_name.to_string(),
                )
            >
                { self.props.children.clone() }
            </form>
        }
    }
}
