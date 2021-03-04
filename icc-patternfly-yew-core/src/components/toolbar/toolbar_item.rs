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
    pub children: Children,
    #[prop_or(ToolbarItemVariant::None)]
    pub variant: ToolbarItemVariant,
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
                    class=(
                        "pf-c-toolbar__item",
                        TOOLBAR_ITEM_VARIANT_STYLES[self.props.variant.clone() as usize],
                    )
                >
                    { self.props.children.clone() }
                </div>
            }
        }
    }
}
