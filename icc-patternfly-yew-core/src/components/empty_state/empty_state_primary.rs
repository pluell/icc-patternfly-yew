use yew::{
    prelude::*,
};


pub struct EmptyStatePrimary
{
    props: EmptyStatePrimaryProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStatePrimaryProps
{
    /** Additional classes added to the EmptyStatePrimary */
    #[prop_or_default]
    pub class_name: String,
    /** Content rendered inside the EmptyStatePrimary */
    #[prop_or_default]
    pub children: Children,
}

impl Component for EmptyStatePrimary
{
    type Message = ();
    type Properties = EmptyStatePrimaryProps;

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
        html!{
            <div 
                class=classes!(
                    "pf-c-empty-state__primary",
                    self.props.class_name.clone()
                ) 
                // {...props}
            >
            {
                for self.props.children.iter()
            }
            </div>
        }
    }
}
