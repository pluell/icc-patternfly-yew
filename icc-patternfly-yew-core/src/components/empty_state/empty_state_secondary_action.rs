use yew::{
    prelude::*,
};


pub struct EmptyStateSecondaryAction
{
    props: EmptyStateSecondaryActionProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateSecondaryActionProps
{
    /** Content rendered inside the EmptyState */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the EmptyState */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for EmptyStateSecondaryAction
{
    type Message = ();
    type Properties = EmptyStateSecondaryActionProps;

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
                    "pf-c-empty-state__secondary",
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
