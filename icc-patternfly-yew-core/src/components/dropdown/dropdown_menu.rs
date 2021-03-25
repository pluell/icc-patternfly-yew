use yew::{
    prelude::*,
};

use super::{DropdownItem, DropdownPosition};


pub struct DropdownMenu
{
    link: ComponentLink<Self>,
    props: DropdownMenuProperties,
}

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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            link,
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
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            DropdownMenuMsg::OnClickDiv => {
                self.props.onselect.emit(());
            }
        }

        false
    }

    fn view(&self) -> Html
    {
        if self.props.component == "div"
        {
            html!{
                <div
                    class=(
                        "pf-c-dropdown__menu",
                        if self.props.position == DropdownPosition::Right { "pf-m-align-right" } else { "" },
                        self.props.class_name.clone()
                    )
                    hidden=!self.props.is_open
                    onclick=self.link.callback(|_| DropdownMenuMsg::OnClickDiv)
                >
                    { for self.props.children.iter() }
                </div>
            }
        }
        else
        {
            if self.props.is_grouped
            {
                // TODO: Render grouped
                html!{}
            }
            else
            {


                html!{
                    <@{self.props.component.clone()}
                        class=(
                            "pf-c-dropdown__menu",
                            if self.props.position == DropdownPosition::Right { "pf-m-align-right" } else { "" },
                            self.props.class_name.clone(),
                        )
                        hidden=!self.props.is_open
                        role="menu"
                    >
                    {
                        for self.props.children.iter().map(|mut child| {
                            child.props.onselect = self.props.onselect.clone();

                            child
                        })
                    }
                    </@>
                }
            }
        }
    }
}
