use yew::{
    prelude::*,
};

use super::{BreadcrumbItem};


pub struct Breadcrumb
{
    props: BreadcrumbProps,
}

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
            <nav
                // {...props}
                aria-label=self.props.aria_label.clone()
                class=classes!(
                    "pf-c-breadcrumb",
                    self.props.class_name.clone(),
                )
                // {...ouiaProps}
            >
                <ol class="pf-c-breadcrumb__list">
                {
                    for self.props.children.iter().enumerate().map(|(index, mut child)|{
                        if index > 0
                        {
                            child.props.show_divider = true;
                        }

                        child
                    })
                }
                </ol>
            </nav>
        }
    }
}
