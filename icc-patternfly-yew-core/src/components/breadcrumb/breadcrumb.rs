use yew::{
    prelude::*,
};

use super::{BreadcrumbItem};


pub struct Breadcrumb;

#[derive(Clone, PartialEq, Properties)]
pub struct BreadcrumbProps
{
    /** Children nodes be rendered to the BreadCrumb. Should be of type BreadCrumbItem. */
    #[prop_or_default]
    pub children: ChildrenWithProps<BreadcrumbItem>,
    /** Additional classes added to the breadcrumb nav. */
    #[prop_or_default]
    pub class_name: String,
    /** Aria label added to the breadcrumb nav. */
    #[prop_or_default]
    pub aria_label: Option<String>,
}

impl Component for Breadcrumb
{
    type Message = ();
    type Properties = BreadcrumbProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <nav
                // {...props}
                aria-label={ctx.props().aria_label.clone()}
                class={classes!(
                    "pf-c-breadcrumb",
                    ctx.props().class_name.clone(),
                )}
                // {...ouiaProps}
            >
                <ol class="pf-c-breadcrumb__list">
                {
                    for ctx.props().children.iter().enumerate().map(|(index, mut child)|{
                        if index > 0
                        {
                            let mut props = (&*child.props).clone();

                            props.show_divider = true;

                            child.props = std::rc::Rc::new(props);
                        }

                        child
                    })
                }
                </ol>
            </nav>
        }
    }
}
