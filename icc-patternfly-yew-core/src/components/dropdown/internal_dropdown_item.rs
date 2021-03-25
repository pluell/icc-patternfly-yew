use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


pub struct InternalDropdownItem
{
    link: ComponentLink<Self>,
    props: InternalDropdownItemProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InternalDropdownItemProperties
{
    /** Anything which can be rendered as dropdown item */
    #[prop_or_default]
    pub children: Children,
    /** Whether to set className on component when React.isValidElement(component) */
    pub style_children: bool,
    /** Classes applied to root element of dropdown item */
    #[prop_or_default]
    pub class_name: String,
    /** Class applied to list element */
    #[prop_or_default]
    pub list_item_class_name: String,
    // /** Indicates which component will be used as dropdown item. Will have className injected if React.isValidElement(component) */
    #[prop_or(String::from("a"))]
    pub component: String,
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
            InternalDropdownItemMsg::OnClick => {
                if !self.props.is_disabled
                {
                    self.props.onclick.emit(());

                    self.props.onselect.emit(());
                }
            }
        };

        false
    }

    fn view(&self) -> Html
    {
        if let Some(custom_child) = &self.props.custom_child
        {
            custom_child.clone()
        }
        else
        {
            html!{
                <li
                    class=self.props.list_item_class_name.clone()
                    role=self.props.role.clone()
                    // onKeyDown={this.onKeyDown}   // TODO
                    onclick=self.link.callback(|_| InternalDropdownItemMsg::OnClick)
                    id=self.props.id.clone()
                >
                {
                    self.render_default_component()
                }
                {
                    // additionalChild && this.extendAdditionalChildRef()
                    if let Some(additional_child) = &self.props.additional_child
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
    fn render_default_component(&self) -> Html
    {
        // Build list of classes
        let mut classes = String::from("pf-c-dropdown__menu-item");

        if self.props.icon.is_some() { classes += " pf-m-icon" }
        if self.props.is_disabled { classes += " pf-m-disabled" }
        if self.props.is_plain_text { classes += " pf-m-plain" }
        if self.props.description.is_some() { classes += " pf-m-description" }
        
        // Add extra classes specified on the parent
        if self.props.class_name.len() > 0
        {
            classes += " ";
            classes += &self.props.class_name;
        }

        // Create the html element
        let mut component_node = VTag::new(self.props.component.clone());

        // Add properties
        component_node.add_attribute("class", &classes);
        component_node.add_attribute("aria-disabled", &self.props.is_disabled);

        if self.props.href.len() > 0
        {
            component_node.add_attribute("href", &self.props.href);
        }

        if self.props.id.len() > 0
        {
            component_node.add_attribute("id", &self.props.component_id);
        }
        
        component_node.add_child(self.render_default_component_content());

        // Convert the VTag into Html
        component_node.into()
    }

    fn render_default_component_content(&self) -> Html
    {
        if let Some(description) = &self.props.description
        {
            html!{
                <>
                    <div class="pf-c-dropdown__menu-item-main">
                    {
                        if let Some(icon) = &self.props.icon
                        {
                            html!{<span class="pf-c-dropdown__menu-item-icon">{icon.clone()}</span>}
                        }
                        else
                        {
                            html!{}
                        }
                    }
                    { self.props.children.clone() }
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
                    if let Some(icon) = &self.props.icon
                    {
                        html!{<span class="pf-c-dropdown__menu-item-icon">{icon.clone()}</span>}
                    }
                    else
                    {
                        html!{}
                    }
                }
                { self.props.children.clone() }
                </>
            }
        }
    }
}