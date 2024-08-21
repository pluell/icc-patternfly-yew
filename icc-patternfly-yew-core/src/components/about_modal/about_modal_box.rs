use yew::prelude::*;


pub struct AboutModalBox;

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalBoxProps
{
    /** content rendered inside the AboutModelBox.  */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the AboutModalBox  */
    #[prop_or_default]
    pub class_name: String,

    // Aria props
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
    #[prop_or_default]
    pub aria_describedby: Option<String>,
}

impl Component for AboutModalBox
{
    type Message = ();
    type Properties = AboutModalBoxProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                role="dialog" aria-modal="true" 
                class={classes!("pf-v5-c-about-modal-box",ctx.props().class_name.clone())}
                // {...props}
                aria-labelledby={ctx.props().aria_labelledby.clone()}
                aria-describedby={ctx.props().aria_describedby.clone()}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}
