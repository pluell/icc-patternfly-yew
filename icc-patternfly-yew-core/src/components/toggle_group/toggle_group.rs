use yew::{
    prelude::*,
};


pub struct ToggleGroup
{
    props: ToggleGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToggleGroupProps
{
    /** Content rendered inside the toggle group */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the toggle group */
    #[prop_or_default]
    pub class_name: String,
    /** Modifies the toggle group to include compact styling. */
    #[prop_or_default]
    pub is_compact: bool,
    /** Accessible label for the toggle group */
    #[prop_or_default]
    pub aria_label: String,
}

impl Component for ToggleGroup
{
    type Message = ();
    type Properties = ToggleGroupProps;

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
                    "pf-c-toggle-group", 
                    if self.props.is_compact { "pf-m-compact" } else { "" },
                    self.props.class_name.clone()
                )
                role="group"
                aria-label=self.props.aria_label.clone()
                // {...props}
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
