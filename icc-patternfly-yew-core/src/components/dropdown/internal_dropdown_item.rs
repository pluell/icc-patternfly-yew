use yew::{
    prelude::*,
    virtual_dom::{VTag},
};

use super::{DropdownItemComponentTypes};


pub struct InternalDropdownItem;

#[derive(Clone, PartialEq, Properties)]
pub struct InternalDropdownItemProperties
{
    /** Anything which can be rendered as dropdown item */
    #[prop_or_default]
    pub children: Children,
    /** Whether to set className on component when React.isValidElement(component) */
    #[prop_or(true)]
    pub style_children: bool,
    /** Classes applied to root element of dropdown item */
    #[prop_or_default]
    pub class_name: String,
    /** Class applied to list element */
    #[prop_or_default]
    pub list_item_class_name: String,
    // /** Indicates which component will be used as dropdown item. Will have className injected if React.isValidElement(component) */
    #[prop_or(DropdownItemComponentTypes::Default("a"))]
    pub component: DropdownItemComponentTypes,
    /** Role for the item */
    #[prop_or_default]
    pub role: String,
    /** Render dropdown item as disabled option */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Render dropdown item as a non-interactive item */
    #[prop_or_default]
    pub is_plain_text: bool,
    /** Forces display of the hover state of the element */
    #[prop_or_default]
    pub is_hovered: bool,
    /** Default hyperlink location */
    #[prop_or_default]
    pub href: String,
    /** Tooltip to display when hovered over the item */
    #[prop_or_default]
    pub tooltip: Option<Html>,
    /** Additional tooltip props forwarded to the Tooltip component */
    #[prop_or_default]
    pub tooltip_props: String,
    #[prop_or_default]
    pub index: i32,
    // #[prop_or_default]
    // context: {
    //     keyHandler: (index: number, innerIndex: number, direction: string) => void;
    //     sendRef: (index: number, ref: any, isDisabled: boolean, isSeparator: boolean) => void;
    // };
    /** Callback for click event */
    #[prop_or_default]
    pub onclick: Callback<()>,  // (event: React.MouseEvent<any> | React.KeyboardEvent | MouseEvent) => void;
    /** ID for the list element */
    #[prop_or_default]
    pub id: String,
    /** ID for the component element */
    #[prop_or_default]
    pub component_id: String,
    /** Additional content to include alongside item within the <li> */
    #[prop_or_default]
    pub additional_child: Option<Html>, // React.ReactNode;
    /** Custom item rendering that receives the DropdownContext */
    #[prop_or_default]
    pub custom_child: Option<Html>, // React.ReactNode;
    /** Flag indicating if hitting enter on an item also triggers an arrow down key press */
    #[prop_or_default]
    pub enter_triggers_arrow_down: bool,
    /** An image to display within the InternalDropdownItem, appearing before any component children */
    #[prop_or_default]
    pub icon: Option<Html>, // React.ReactNode;
    /** Initial focus on the item when the menu is opened (Note: Only applicable to one of the items) */
    #[prop_or_default]
    pub auto_focus: bool,
    /** A short description of the dropdown item, displayed under the dropdown item content */
    #[prop_or_default]
    pub description: Option<Html>,  //React.ReactNode;
    // /** Events to prevent when the item is disabled */
    // #[prop_or_default]
    // pub inoperable_events: Vec<String>, //string[];
    /** Function callback called when user selects item */
    #[prop_or_default]
    pub onselect: Callback<()>,
}

pub enum InternalDropdownItemMsg
{
    OnClick,
}

impl Component for InternalDropdownItem
{
    type Message = InternalDropdownItemMsg;
    type Properties = InternalDropdownItemProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            InternalDropdownItemMsg::OnClick => {
                if !ctx.props().is_disabled
                {
                    ctx.props().onclick.emit(());

                    ctx.props().onselect.emit(());
                }
            }
        };

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        if let Some(custom_child) = &ctx.props().custom_child
        {
            custom_child.clone()
        }
        else
        {
            html!{
                <li
                    class={ctx.props().list_item_class_name.clone()}
                    role={ctx.props().role.clone()}
                    // onKeyDown={this.onKeyDown}   // TODO
                    onclick={ctx.link().callback(|_| InternalDropdownItemMsg::OnClick)}
                    id={ctx.props().id.clone()}
                >
                {
                    match &ctx.props().component
                    {
                        DropdownItemComponentTypes::Custom(component) => {
                            component.clone()
                        },
                        DropdownItemComponentTypes::Default(tag) => {
                            self.view_default_component(ctx, tag.to_string())
                        },
                    }
                }
                {
                    // additionalChild && this.extendAdditionalChildRef()
                    if let Some(additional_child) = &ctx.props().additional_child
                    {
                        html!{additional_child.clone()}
                    }
                    else
                    {
                        html!{}
                    }
                }
                </li>
            }
        }
    }
}

impl InternalDropdownItem
{
    fn view_default_component(&self, ctx: &Context<Self>, tag: String) -> Html
    {
        // Build list of classes
        let mut classes = String::from("pf-c-dropdown__menu-item");

        if ctx.props().icon.is_some() { classes += " pf-m-icon" }
        if ctx.props().is_disabled { classes += " pf-m-disabled" }
        if ctx.props().is_plain_text { classes += " pf-m-plain" }
        if ctx.props().description.is_some() { classes += " pf-m-description" }
        
        // Add extra classes specified on the parent
        if ctx.props().class_name.len() > 0
        {
            classes += " ";
            classes += &ctx.props().class_name;
        }

        // Create the html element
        let mut component_node = VTag::new(tag);

        // Add properties
        component_node.add_attribute("class", classes);
        component_node.add_attribute("aria-disabled", ctx.props().is_disabled.to_string());

        if ctx.props().href.len() > 0
        {
            component_node.add_attribute("href", ctx.props().href.clone());
        }

        if ctx.props().id.len() > 0
        {
            component_node.add_attribute("id", ctx.props().component_id.clone());
        }
        
        component_node.add_child(self.view_default_component_content(ctx));

        // Convert the VTag into Html
        component_node.into()
    }

    fn view_default_component_content(&self, ctx: &Context<Self>) -> Html
    {
        if let Some(description) = &ctx.props().description
        {
            html!{
                <>
                    <div class="pf-c-dropdown__menu-item-main">
                    {
                        if let Some(icon) = &ctx.props().icon
                        {
                            html!{<span class="pf-c-dropdown__menu-item-icon">{icon.clone()}</span>}
                        }
                        else
                        {
                            html!{}
                        }
                    }
                    { ctx.props().children.clone() }
                    </div>
                    <div class="pf-c-dropdown__menu-item-description">{description.clone()}</div>
                </>
            }
        }
        else
        {
            html!{
                <>
                {
                    if let Some(icon) = &ctx.props().icon
                    {
                        html!{<span class="pf-c-dropdown__menu-item-icon">{icon.clone()}</span>}
                    }
                    else
                    {
                        html!{}
                    }
                }
                { ctx.props().children.clone() }
                </>
            }
        }
    }
}