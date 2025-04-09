use yew::prelude::*;


pub struct EmptyStateIcon;

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateIconProps
{
    /** Additional classes added to the empty state icon */
    #[prop_or_default]
    pub classes: Classes,
    /** Icon component to be rendered. Can also be a spinner component */
    pub icon: Html,
    /** Changes the color of the icon.  */
    #[prop_or_default]
    pub color: Option<AttrValue>,
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
        html!{
            <div
                class="pf-v5-c-empty-state__icon"
                // {...(color && !iconIsSpinner && { style: { [cssIconColor.name]: color } as React.CSSProperties })}
                style={
                    if let Some(color) = &ctx.props().color {
                        Some(format!("color: {}", color))
                    } else {
                        None
                    }
                }
            >
                // <IconComponent className={className} aria-hidden={!iconIsSpinner} {...props} />
                {ctx.props().icon.clone()}
            </div>
        }
    }
}
