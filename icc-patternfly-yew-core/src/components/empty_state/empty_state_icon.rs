use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


#[derive(Clone, PartialEq)]
pub enum EmptyStateIconVariants
{
    Icon,
    Container,
}

pub struct EmptyStateIcon
{
    props: EmptyStateIconProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateIconProps
{
    /** Additional classes added to the EmptyState */
    #[prop_or_default]
    pub class_name: String,
    /** Icon component to be rendered inside the EmptyState on icon variant
     * Usually a CheckCircleIcon, ExclamationCircleIcon, LockIcon, PlusCircleIcon, RocketIcon
     * SearchIcon, or WrenchIcon */
     #[prop_or_default]
    pub icon: Option<VTag>,
    /** Component to be rendered inside the EmptyState on container variant */
    #[prop_or_default]
    pub component: Option<Html>,
    /** Adds empty state icon variant styles  */
    #[prop_or(EmptyStateIconVariants::Icon)]
    pub variant: EmptyStateIconVariants,
}

impl Component for EmptyStateIcon
{
    type Message = ();
    type Properties = EmptyStateIconProps;

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
        let mut classes = classes!("pf-c-empty-state__icon", self.props.class_name.clone());

        match self.props.variant
        {
            EmptyStateIconVariants::Icon => {
                if let Some(icon) = &self.props.icon
                {
                    // Find class attribue
                    for (attr, value) in icon.attributes.iter()
                    {
                        if attr == "class"
                        {   
                            // Add the icon classes to the common classes
                            classes = classes!(value.to_string(), classes);
                        }
                    }

                    let mut icon_node = icon.clone();
                    icon_node.add_attribute("class", classes.to_string());
                    icon_node.into()
                }
                else
                {
                    html!{}
                }
            },
            EmptyStateIconVariants::Container => {
                html!{
                    <div class=classes>
                    {
                        if let Some(component) = &self.props.component
                        {
                            component.clone()
                        }
                        else
                        {
                            html!{}
                        }
                    }
                    </div>
                }
            }
        }
    }
}
