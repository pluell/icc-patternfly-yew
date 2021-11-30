use yew::{
    prelude::*,
};


pub struct MastheadMain
{
    props: MastheadMainProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MastheadMainProps
{
    /** Content rendered inside of the masthead main block. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the masthead main. */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for MastheadMain
{
    type Message = ();
    type Properties = MastheadMainProps;

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
        true
    }

    fn view(&self) -> Html
    {
        html!{
            <div 
                class=classes!(
                    "pf-c-masthead__main",
                    self.props.class_name.clone(),
                )
                // {...props}
            >
                {for self.props.children.iter()}
            </div>
        }
    }
}
