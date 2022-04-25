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

pub struct EmptyStateIcon;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut classes = classes!("pf-c-empty-state__icon", ctx.props().class_name.clone());

        match ctx.props().variant
        {
            EmptyStateIconVariants::Icon => {
                if let Some(icon) = &ctx.props().icon
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
                    <div class={classes}>
                    {
                        if let Some(component) = &ctx.props().component
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
