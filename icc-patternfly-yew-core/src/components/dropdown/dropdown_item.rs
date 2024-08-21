use yew::prelude::*;

use super::{DropdownItemComponentTypes, InternalDropdownItem};


pub struct DropdownItem;

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownItemProperties
{
    /** Anything which can be rendered as dropdown item */
    #[prop_or_default]
    pub children: Children,
    /** Classes applied to root element of dropdown item */
    #[prop_or_default]
    pub class_name: String,
    /** Class to be applied to list item */
    #[prop_or_default]
    pub list_item_class_name: String,
    // /**
    //  * A ReactElement to render, or a string to use as the component tag.
    //  * Example: component={<Link to="/components/alert/">Alert</Link>}
    //  * Example: component="button"
    //  * If React.isValidElement(component) the className prop will be injected unless styleChildren="false"
    //  */
    #[prop_or(DropdownItemComponentTypes::Default("a"))]
    pub component: DropdownItemComponentTypes,
    /** Whether to set className on component when React.isValidElement(component) */
    #[prop_or_default]
    pub style_children: bool,
    /** Render dropdown item as disabled option */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Render dropdown item as non-interactive item */
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
    /** Additional node to include alongside item within the <li> */
    #[prop_or_default]
    pub additional_child: Option<Html>, // React.ReactNode;
    /** Custom item rendering that receives the DropdownContext */
    #[prop_or_default]
    pub custom_child: Option<Html>, // React.ReactNode;
    /** tabIndex to use, null to unset it */
    #[prop_or_default]
    pub tab_index: Option<i32>, //number | null;
    /** An image to display within the DropdownItem, appearing before any component children */
    #[prop_or_default]
    pub icon: Option<Html>, // React.ReactNode;
    /** Initial focus on the item when the menu is opened (Note: Only applicable to one of the items) */
    #[prop_or_default]
    pub auto_focus: bool,
    /** A short description of the dropdown item, displayed under the dropdown item content */
    #[prop_or_default]
    pub description: Option<Html>,  //React.ReactNode;
    /** Callback for click event */
    #[prop_or_default]
    pub onclick: Callback<()>,
    /** Function callback called when user selects item */
    #[prop_or_default]
    pub onselect: Callback<()>,
}

impl Component for DropdownItem
{
    type Message = ();
    type Properties = DropdownItemProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <InternalDropdownItem
                // context={context} // TODO
                role={"menuitem"}
                // tab_index={tabIndex}
                class_name={ctx.props().class_name.clone()}
                component={ctx.props().component.clone()}
                is_disabled={ctx.props().is_disabled}
                is_plain_text={ctx.props().is_plain_text}
                is_hovered={ctx.props().is_hovered}
                href={ctx.props().href.clone()}
                tooltip={ctx.props().tooltip.clone()}
                tooltip_props={ctx.props().tooltip_props.clone()}
                list_item_class_name={ctx.props().list_item_class_name.clone()}
                onclick={ctx.props().onclick.clone()} // TODO
                additional_child={ctx.props().additional_child.clone()}
                custom_child={ctx.props().custom_child.clone()}
                icon={ctx.props().icon.clone()}
                auto_focus={ctx.props().auto_focus}
                style_children={ctx.props().style_children}
                description={ctx.props().description.clone()}
                // {...ouiaProps}
                // {...props}
                onselect={ctx.props().onselect.clone()}
            >
            { ctx.props().children.clone() }
            </InternalDropdownItem>
        }
    }
}
