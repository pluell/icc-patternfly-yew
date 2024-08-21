use yew::{
    prelude::*,
    virtual_dom::VTag,
};


pub struct BreadcrumbItemRenderArgs
{
    pub class_name: String,
    pub aria_current: Option<&'static str>,
}


pub struct BreadcrumbItem;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let aria_current = if ctx.props().is_active { Some("page") } else {None};
        let classes = classes!(
                        "pf-v5-c-breadcrumb__link",
                        if ctx.props().is_active {"pf-m-current"} else {""}
                    );

        html!{
            <li
                //{...props}
                class={classes!(
                    "pf-v5-c-breadcrumb__item",
                    ctx.props().class_name.clone(),
                )}
            >
            {
                if ctx.props().show_divider
                {
                    html!{
                        <span class="pf-v5-c-breadcrumb__item-divider">
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
                if ctx.props().component == "button"
                {
                    html!{
                        <button
                            class={classes}
                            aria-current={aria_current}
                            type="button"
                        >
                            {for ctx.props().children.iter()}
                        </button>
                    }
                }
                else if ctx.props().is_dropdown
                {
                    html!{
                        <span class="pf-v5-c-breadcrumb__dropdown">
                            {for ctx.props().children.iter()}
                        </span>
                    }
                }
                else if let Some(render) = &ctx.props().render
                {
                    render(BreadcrumbItemRenderArgs{
                        class_name: classes.to_string(),
                        aria_current,
                    })
                }
                else if let Some(to) = &ctx.props().to
                {
                    let mut component = VTag::new(ctx.props().component.clone());
            
                    component.add_attribute("href", to.to_string());

                    if let Some(target) = &ctx.props().target
                    {
                        component.add_attribute("target", target.to_string());
                    }

                    component.add_attribute("class", classes.to_string());
                    
                    if let Some(aria_current) = aria_current
                    {
                        component.add_attribute("aria-current", aria_current.to_string());
                    }
                            
                    component.add_children(ctx.props().children.iter());
            
                    component.into()
                }
                else
                {
                    html!{
                        for ctx.props().children.iter()
                    }
                }
            }
            </li>
        }
    }
}
