use yew::prelude::*;

pub struct HelperText;

#[derive(Clone, PartialEq, Properties)]
pub struct HelperTextProperties
{
    /** Content rendered inside the helper text container. */
    pub children: Children,
    /** Additional classes applied to the helper text container. */
    #[prop_or_default]
    pub classes: Classes,
    /** Component type of the helper text container */
    #[prop_or(String::from("div"))]
    pub component: String,
    /** ID for the helper text container. The value of this prop can be passed into a form component's
     * aria-describedby prop when you intend for all helper text items to be announced to
     * assistive technologies.
     */
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /** Flag for indicating whether the helper text container is a live region. Use this prop when you
     * expect or intend for any helper text items within the container to be dynamically updated.
     */
    #[prop_or_default]
    pub is_live_region: bool,
    /** Adds an accessible label to the helper text when component is a "ul". */
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
}

impl Component for HelperText
{
    type Message = ();
    type Properties = HelperTextProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <@{ctx.props().component.clone()}
                id={ctx.props().id.clone()}
                class={classes!(
                    "pf-v5-c-helper-text",
                    ctx.props().classes.clone()
                )}
                aria-live={if ctx.props().is_live_region {"polite"} else {""}}
                aria-label={ctx.props().aria_label.clone()}
                role={if ctx.props().component == "ul" {"list"} else {""}}
                // {...props}
            >
                { ctx.props().children.clone() }
            </@>
        }
    }
}
