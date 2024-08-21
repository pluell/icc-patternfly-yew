use yew::prelude::*;


pub struct Thead;

#[derive(Clone, PartialEq, Properties)]
pub struct TheadProps
{
    /** Content rendered inside the <thead> row group */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the <thead> element */
    #[prop_or_default]
    pub class_name: String,
    /** Won't wrap the table head if true */
    #[prop_or_default]
    pub no_wrap: bool,
    // /** Forwarded ref */
    // innerRef?: React.Ref<any>;
    /** Indicates the <thead> contains a nested header */
    #[prop_or_default]
    pub has_nested_header: bool,
}

impl Component for Thead
{
    type Message = ();
    type Properties = TheadProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <thead
                class={classes!(
                    &ctx.props().class_name,
                    if ctx.props().no_wrap {"pf-m-nowrap"} else {""},
                    if ctx.props().has_nested_header {"pf-m-nested-column-header"} else {""}
                )}
                // ref={innerRef}
                // {...props}
            >
            { for ctx.props().children.iter() }
            </thead>
        }
    }
}
