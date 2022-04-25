use yew::{
    prelude::*,
};


pub struct InputGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProperties
{
    /** Additional classes added to the input group. */
    #[prop_or_default]
    pub class_name: String,
    /** Content rendered inside the input group. */
    #[prop_or_default]
    pub children: Children,
}

impl Component for InputGroup
{
    type Message = ();
    type Properties = InputGroupProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-c-input-group", 
                    &ctx.props().class_name,
                )}
                // {...props}
            >
            {
                // {idItem
                // ? React.Children.map(children, (child: any) =>
                //     formCtrls.includes(child.type.displayName)
                //         ? React.cloneElement(child, { 'aria-describedby': idItem.props.id })
                //         : child
                //     )
                // : children}
                for ctx.props().children.iter()
            }
            </div>
        }
    }
}
