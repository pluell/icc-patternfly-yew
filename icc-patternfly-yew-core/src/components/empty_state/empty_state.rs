use yew::{
    prelude::*,
};


#[derive(Clone, PartialEq)]
pub enum EmptyStateVariant
{
    Xs,
    Small,
    Large,
    Xl,
    Full,
}

pub struct EmptyState
{
    props: EmptyStateProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateProps
{
    /** Additional classes added to the EmptyState */
    #[prop_or_default]
    pub class_name: String,
    /** Content rendered inside the EmptyState */
    #[prop_or_default]
    pub children: Children,
    /** Modifies EmptyState max-width */
    #[prop_or(EmptyStateVariant::Full)]
    pub variant: EmptyStateVariant,
    /** Cause component to consume the available height of its container */
    #[prop_or_default]
    pub is_full_height: bool,
}

impl Component for EmptyState
{
    type Message = ();
    type Properties = EmptyStateProps;

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
        let variant_cls = match self.props.variant {
            EmptyStateVariant::Xs => "pf-m-xs",
            EmptyStateVariant::Small => "pf-m-sm",
            EmptyStateVariant::Large => "pf-m-lg",
            EmptyStateVariant::Xl => "pf-m-xl",
            EmptyStateVariant::Full => "",
        };

        html!{
            <div
                class=classes!(
                    "pf-c-empty-state",
                    variant_cls,
                    if self.props.is_full_height { "pf-m-full-height" } else { "" },
                    self.props.class_name.clone()
                )
                // {...props}
            >
                <div class="pf-c-empty-state__content">
                {
                    for self.props.children.iter()
                }
                </div>
            </div>
        }
    }
}
