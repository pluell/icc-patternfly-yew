use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


pub struct BreadcrumbHeading
{
    props: BreadcrumbHeadingProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BreadcrumbHeadingProps
{
    /** Content rendered inside the breadcrumb title. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the breadcrumb item. */
    #[prop_or_default]
    pub class_name: String,
    /** HREF for breadcrumb link. */
    #[prop_or_default]
    pub to: Option<String>,
    /** Target for breadcrumb link. */
    #[prop_or_default]
    pub target: Option<String>,
    /** Sets the base component to render. Defaults to <a> */
    #[prop_or(String::from("a"))]
    pub component: String,
    /** Internal prop set by Breadcrumb on all but the first crumb */
    #[prop_or_default]
    pub show_divider: bool,
}

impl Component for BreadcrumbHeading
{
    type Message = ();
    type Properties = BreadcrumbHeadingProps;

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
                
                <h1 class="pf-c-breadcrumb__heading">
                {
                    if self.props.to.is_none() && self.props.component == "button"
                    {
                        html!{
                            <button 
                                class=classes!(
                                    "pf-c-breadcrumb__link",
                                    "pf-m-current",
                                )
                                aria-current=true.to_string()
                                type="button"
                            >
                                {for self.props.children.iter()}
                            </button>
                        }
                    }
                    else if let Some(to) = &self.props.to
                    {
                        let mut component = VTag::new(self.props.component.clone());

                        component.add_attribute("href", to.to_string());

                        if let Some(target) = &self.props.target
                        {
                            component.add_attribute("target", target.to_string());
                        }
                
                        component.add_attribute("class", "pf-c-breadcrumb__link pf-m-current");
                        component.add_attribute("aria-current", "page");
                                
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
                </h1>
            </li>
        }
    }
}
