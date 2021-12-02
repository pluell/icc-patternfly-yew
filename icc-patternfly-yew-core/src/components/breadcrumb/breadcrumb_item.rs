use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


pub struct BreadcrumbItemRenderArgs
{
    pub class_name: String,
    pub aria_current: Option<&'static str>,
}


pub struct BreadcrumbItem
{
    props: BreadcrumbItemProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BreadcrumbItemProps
{
    /** Content rendered inside the breadcrumb item. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the breadcrumb item. */
    #[prop_or_default]
    pub class_name: String,
    /** HREF for breadcrumb link. */
    #[prop_or_default]
    pub to: Option<String>,
    /** Flag indicating whether the item is active. */
    #[prop_or_default]
    pub is_active: bool,
    /** Flag indicating whether the item contains a dropdown. */
    #[prop_or_default]
    pub is_dropdown: bool,
    /** Internal prop set by Breadcrumb on all but the first crumb */
    #[prop_or_default]
    pub show_divider: bool,
    /** Target for breadcrumb link. */
    #[prop_or_default]
    pub target: Option<String>,
    /** Sets the base component to render. Defaults to <a> */
    #[prop_or(String::from("a"))]
    pub component: String,
    /** A render function to render the component inside the breadcrumb item. */
    #[prop_or_default]
    pub render: Option<fn(BreadcrumbItemRenderArgs) -> Html>,
}

impl Component for BreadcrumbItem
{
    type Message = ();
    type Properties = BreadcrumbItemProps;

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
        let aria_current = if self.props.is_active { Some("page") } else {None};
        let classes = classes!(
                        "pf-c-breadcrumb__link",
                        if self.props.is_active {"pf-m-current"} else {""}
                    );

        html!{
            <li
                //{...props}
                class=classes!(
                    "pf-c-breadcrumb__item",
                    self.props.class_name.clone(),
                )
            >
            {
                if self.props.show_divider
                {
                    html!{
                        <span class="pf-c-breadcrumb__item-divider">
                            {icc_patternfly_yew_icons::angle_right_icon!{}}
                        </span>
                    }
                }
                else
                {
                    html!{}
                }
            }
            {
                if self.props.component == "button"
                {
                    html!{
                        <button
                            class=classes
                            aria-current=aria_current
                            type="button"
                        >
                            {for self.props.children.iter()}
                        </button>
                    }
                }
                else if self.props.is_dropdown
                {
                    html!{
                        <span class="pf-c-breadcrumb__dropdown">
                            {for self.props.children.iter()}
                        </span>
                    }
                }
                else if let Some(render) = &self.props.render
                {
                    render(BreadcrumbItemRenderArgs{
                        class_name: classes.to_string(),
                        aria_current,
                    })
                }
                else if let Some(to) = &self.props.to
                {
                    let mut component = VTag::new(self.props.component.clone());
            
                    component.add_attribute("href", to.to_string());

                    if let Some(target) = &self.props.target
                    {
                        component.add_attribute("target", target.to_string());
                    }

                    component.add_attribute("class", classes.to_string());
                    
                    if let Some(aria_current) = aria_current
                    {
                        component.add_attribute("aria-current", aria_current.to_string());
                    }
                            
                    component.add_children(self.props.children.iter());
            
                    component.into()
                }
                else
                {
                    html!{
                        for self.props.children.iter()
                    }
                }
            }
            </li>
        }
    }
}
