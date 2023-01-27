use yew::{
    prelude::*,
};


pub struct Tbody;

#[derive(Clone, PartialEq, Properties)]
pub struct TbodyProps
{
    /** Content rendered inside the <tbody> row group */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the <tbody> element  */
    #[prop_or_default]
    pub class_name: String,
    /** Modifies the body to allow for expandable rows */
    #[prop_or_default]
    pub is_expanded: bool,
    // /** Forwarded ref */
    // innerRef?: React.Ref<any>;
    /** Flag indicating the <tbody> contains oddly striped rows. */
    #[prop_or_default]
    pub is_odd_striped: bool,
    /** Flag indicating the <tbody> contains evenly striped rows. */
    #[prop_or_default]
    pub is_even_striped: bool,
}

impl Component for Tbody
{
    type Message = ();
    type Properties = TbodyProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <tbody
                role="rowgroup"
                class={classes!(
                    &ctx.props().class_name,
                    if ctx.props().is_expanded {"styles.modifiers.expanded"} else {""},
                    if ctx.props().is_odd_striped {"styles.modifiers.striped"} else {""},
                    if ctx.props().is_even_striped {"styles.modifiers.stripedEven"} else {""},
                )}
                // ref={innerRef}
                // {...props}
            >
            { for ctx.props().children.iter() }
            </tbody>
        }
    }
}
