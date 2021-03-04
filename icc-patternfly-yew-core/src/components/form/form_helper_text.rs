use yew::{
    prelude::*,
};

pub struct FormHelperText
{
    props: FormHelperTextProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormHelperTextProperties
{
    /** Content rendered inside the Helper Text Item */
    pub children: Children,
    /** Adds error styling to the Helper Text  * */
    #[prop_or_default]
    pub is_error: bool,
    /** Hides the helper text * */
    #[prop_or_default]
    pub is_hidden: bool,
    /** Additional classes added to the Helper Text Item  */
    #[prop_or_default]
    pub class_name: String,
    /** Icon displayed to the left of the helper text. */
    #[prop_or_default]
    pub icon: Option<Html>,
}

impl Component for FormHelperText
{
    type Message = ();
    type Properties = FormHelperTextProperties;

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
            <p
                class=(
                    "pf-c-form__helper-text",
                    if self.props.is_error {"pf-m-error"} else {""},
                    if self.props.is_hidden {"pf-m-hidden"} else {""},
                    self.props.class_name.to_string()
                )
                // {...props}
            >
                {
                    if let Some(icon) = &self.props.icon
                    {
                        html!{
                            <span class="pf-c-form__helper-text-icon">{icon.clone()}</span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                { self.props.children.clone() }
            </p>
        }
    }
}
