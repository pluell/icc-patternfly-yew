use std::collections::{HashMap};

use yew::{
    prelude::*,
};

use super::*;


pub struct Toolbar
{
    props: ToolbarProperties,
    link: ComponentLink<Toolbar>,
    chip_group_content_ref: NodeRef,
    filter_info: HashMap<String, i32>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarProperties
{
    /** Optional callback for clearing all filters in the toolbar */
    #[prop_or_default]
    pub clear_all_filters: Callback<()>,
    /** Text to display in the clear all filters button */
    #[prop_or_default]
    pub clear_filters_button_text: String,
    // /** The breakpoint at which the listed fitlers in chip groups are collapsed down to a summary */
    // collapseListedFiltersBreakpoint?: 'all' | 'md' | 'lg' | 'xl' | '2xl';
    /** Flag indicating if a data toolbar toggle group's expandable content is expanded */
    #[prop_or_default]
    pub is_expanded: bool,
    /** A callback for setting the isExpanded flag */
    #[prop_or_default]
    pub toggle_is_expanded: Option<Callback<()>>,
    /** Classes applied to root element of the data toolbar */
    #[prop_or_default]
    pub class_name: String,
    /** Content to be rendered as rows in the data toolbar */
    pub children: ChildrenWithProps<ToolbarContent>,
    /** Id of the data toolbar */
    #[prop_or_default]
    pub id: String,
    // /** Flag indicating the toolbar should use the Page insets */
    // pub use_page_insets: bool,
}

pub enum ToolbarMsg
{
    UpdateNumFilters((String, i32)),
}

impl Component for Toolbar
{
    type Message = ToolbarMsg;
    type Properties = ToolbarProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            props,
            link,
            chip_group_content_ref: NodeRef::default(),
            filter_info: HashMap::new(),
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
            ToolbarMsg::UpdateNumFilters((group_name, num_chips)) => {
                self.filter_info.insert(group_name, num_chips);
            },
        }

        true
    }

    fn view(&self) -> Html
    {
        // Calculate number of filters
        let number_of_filters = self.get_num_filters();

        html!{
            <div
                id=&self.props.id
                class="pf-c-toolbar"
            >
                {
                    for self.props.children.iter().map(|mut content_node| {
                        content_node.props.chip_group_content_ref = self.chip_group_content_ref.clone();
                        content_node.props.update_number_filters = self.link.callback(|(group_name, num_chips)| 
                            ToolbarMsg::UpdateNumFilters((group_name, num_chips))
                        );

                        content_node
                    })
                }
                <ToolbarChipGroupContent 
                    ref=self.chip_group_content_ref.clone()
                    show_clear_filters_button=(number_of_filters > 0)
                    clear_filters_button_text=self.props.clear_filters_button_text.clone()
                    clear_all_filters=self.props.clear_all_filters.clone()
                    number_of_filters=number_of_filters
                />
            </div>
        }
    }
}

impl Toolbar
{
    fn get_num_filters(&self) -> i32
    {
        let mut num_filters = 0;

        for (_, num_chips) in self.filter_info.iter()
        {
            num_filters += num_chips;
        }

        num_filters
    }
}
