use yew::{
    prelude::*,
};

use super::{DropdownItem, DropdownPosition};


pub struct DropdownMenu;

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownMenuProperties
{
    /** Anything which can be rendered as dropdown items */
    #[prop_or_default]
    pub children: ChildrenWithProps<DropdownItem>,
    /** Classess applied to root element of dropdown menu */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to indicate if menu is opened */
    #[prop_or_default]
    pub is_open: bool,
    /** Flag to indicate if the first dropdown item should gain initial focus, set false when adding
     * a specific auto-focus item (like a current selection) otherwise leave as true
     */
    #[prop_or_default]
    pub auto_focus: bool,
    /** Indicates which component will be used as dropdown menu */
    #[prop_or(String::from("ul"))]
    pub component: String,  //React.ReactNode;
    /** Indicates where menu will be alligned horizontally */
    #[prop_or(DropdownPosition::Left)]
    pub position: DropdownPosition,
    /** Flag to indicate if menu is grouped */
    #[prop_or_default]
    pub is_grouped: bool,
    /** Function callback called when user selects item */
    #[prop_or_default]
    pub onselect: Callback<()>,
}

pub enum DropdownMenuMsg
{
    OnClickDiv,
}

impl Component for DropdownMenu
{
    type Message = DropdownMenuMsg;
    type Properties = DropdownMenuProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            DropdownMenuMsg::OnClickDiv => {
                ctx.props().onselect.emit(());
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        if ctx.props().component == "div"
        {
            html!{
                <div
                    class={classes!(
                        "pf-c-dropdown__menu",
                        if ctx.props().position == DropdownPosition::Right { "pf-m-align-right" } else { "" },
                        ctx.props().class_name.clone()
                    )}
                    hidden={!ctx.props().is_open}
                    onclick={ctx.link().callback(|_| DropdownMenuMsg::OnClickDiv)}
                >
                    { for ctx.props().children.iter() }
                </div>
            }
        }
        else
        {
            if ctx.props().is_grouped
            {
                // TODO: Render grouped
                html!{}
            }
            else
            {


                html!{
                    <@{ctx.props().component.clone()}
                        class={classes!(
                            "pf-c-dropdown__menu",
                            if ctx.props().position == DropdownPosition::Right { "pf-m-align-right" } else { "" },
                            ctx.props().class_name.clone(),
                        )}
                        hidden={!ctx.props().is_open}
                        role={"menu"}
                    >
                    {
                        for ctx.props().children.iter().map(|mut child| {
                            let mut props = (&*child.props).clone();
                            
                            props.onselect = ctx.props().onselect.clone();

                            child.props = std::rc::Rc::new(props);

                            child
                        })
                    }
                    </@>
                }
            }
        }
    }
}
