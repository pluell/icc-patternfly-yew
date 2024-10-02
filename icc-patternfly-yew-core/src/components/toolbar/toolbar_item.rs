use yew::prelude::*;

use crate::Divider;


#[derive(Clone, PartialEq)]
pub enum ToolbarItemVariant
{
    None,
    Separator,
    BulkSelect,
    OverflowMenu,
    Pagination,
    SearchFilter,
    Label,
    ChipGroup,
    ExpandAll,
}

const TOOLBAR_ITEM_VARIANT_STYLES: &'static [&'static str] = &[
    "",
    "",
    "pf-m-bulk-select",
    "pf-m-overflow-menu",
    "pf-m-pagination",
    "pf-m-search-filter",
    "pf-m-label",
    "pf-m-chip-group",
    "pf-m-expand-all",
];

pub struct ToolbarItem;

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarItemProperties
{
    /** Classes applied to root element of the data toolbar item */
    #[prop_or_default]
    pub class_name: String,
    /** A type modifier which modifies spacing specifically depending on the type of item */
    #[prop_or(ToolbarItemVariant::None)]
    pub variant: ToolbarItemVariant,
    /** id for this data toolbar item */
    #[prop_or_default]
    pub id: String,
    /** Flag indicating if the expand-all variant is expanded or not */
    #[prop_or_default]
    pub is_all_expanded: bool,
    /** Content to be rendered inside the data toolbar item */
    #[prop_or_default]
    pub children: Children,

    // Extra properties
    #[prop_or_default]
    pub filter_item_node: NodeRef,
}

impl Component for ToolbarItem
{
    type Message = ();
    type Properties = ToolbarItemProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        if ctx.props().variant == ToolbarItemVariant::Separator
        {
            html!{<Divider is_vertical=true />}
        }
        else
        {
            html!{
                <div 
                    ref={&ctx.props().filter_item_node}
                    id={ctx.props().id.clone()}
                    class={classes!(
                        "pf-v5-c-toolbar__item",
                        TOOLBAR_ITEM_VARIANT_STYLES[ctx.props().variant.clone() as usize],
                        &ctx.props().class_name,
                    )}
                >
                    { ctx.props().children.clone() }
                </div>
            }
        }
    }
}
