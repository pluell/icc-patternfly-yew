use yew::prelude::*;


pub struct PageSidebarBody;

#[derive(Clone, PartialEq, Properties)]
pub struct PageSidebarBodyProps
{
    /** Content rendered inside the page sidebar body */
    #[prop_or_default]
    pub classes: Classes,
    /** Additional classes added to the page sidebar body */
    #[prop_or_default]
    pub children: Html,
    /** Flag indicating that the page sidebar body should use page insets. */
    #[prop_or_default]
    pub use_page_insets: bool,
    /** Flag indicating that the page sidebar body should fill the available vertical space. */
    #[prop_or_default]
    pub is_filled: Option<bool>,
}

impl Component for PageSidebarBody
{
    type Message = ();
    type Properties = PageSidebarBodyProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let fill_cls = if let Some(is_filled) = ctx.props().is_filled {
            if is_filled {
                "pf-m-fill"
            } else {
                "pf-m-no-fill"
            }
        } else {
            ""
        };

        html!{
            <div
                class={classes!(
                    "pf-v5-c-page__sidebar-body",
                    if ctx.props().use_page_insets {"pf-m-page-insets"} else {""},
                    fill_cls,
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
