use yew::{
    prelude::*,
};


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

pub struct ToolbarItem
{
    props: ToolbarItemProperties,
}

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
}

impl Component for ToolbarItem
{
    type Message = ();
    type Properties = ToolbarItemProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
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
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        true
    }

    fn view(&self) -> Html
    {
        if self.props.variant == ToolbarItemVariant::Separator
        {
            // TODO: Implement Divider component
            html!{}
        }
        else
        {
            html!{
                <div 
                    id=&self.props.id
                    class=(
                        "pf-c-toolbar__item",
                        TOOLBAR_ITEM_VARIANT_STYLES[self.props.variant.clone() as usize],
                        &self.props.class_name,
                    )
                >
                    { self.props.children.clone() }
                </div>
            }
        }
    }
}
