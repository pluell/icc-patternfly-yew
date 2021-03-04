use yew::{
    prelude::*,
};

use crate::components::{Button, ButtonVariant, ToolbarItem, ToolbarGroup};


// 'all' | 'md' | 'lg' | 'xl' | '2xl'
#[derive(Clone, PartialEq)]
pub enum CollapseFiltersBreakpoint
{
    None,
    All,
    Md,
    Lg,
    Xl,
    Xxl,
}

pub struct ToolbarChipGroupContent
{
    link: ComponentLink<Self>,
    props: ToolbarChipGroupContentProperties,
}

pub enum ToolbarChipGroupContentMsg
{
    ClearChipGroups,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarChipGroupContentProperties
{
    #[prop_or_default]
    pub is_expanded: bool,
    #[prop_or_default]
    pub show_clear_filters_button: bool,
    #[prop_or_default]
    pub clear_filters_button_text: String,
    #[prop_or_default]
    pub number_of_filters: i32,
    #[prop_or(CollapseFiltersBreakpoint::None)]
    pub collapse_listed_filters_breakpoint: CollapseFiltersBreakpoint,
    #[prop_or_default]
    pub clear_all_filters: Callback<()>,
}

impl Component for ToolbarChipGroupContent
{
    type Message = ToolbarChipGroupContentMsg;
    type Properties = ToolbarChipGroupContentProperties;

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
            ToolbarChipGroupContentMsg::ClearChipGroups => {
                self.props.clear_all_filters.emit(());

                false
            }
        }
    }

    fn view(&self) -> Html
    {
        let mut collapse_filters = false;
        if self.props.collapse_listed_filters_breakpoint == CollapseFiltersBreakpoint::All
        {
            collapse_filters = true;
        }
        // TODO: Implement breakpoint based on window size
        // else if (typeof window !== 'undefined')
        // {
        //     collapse_filters = window.innerWidth < globalBreakpoints[collapseListedFiltersBreakpoint];
        // }

        html!{
            <div class="pf-c-toolbar__content">
                // TODO: Implement collapse listed filters
                <ToolbarGroup 
                    class_name={if collapse_filters {"pf-m-hidden"} else {""}}
                    // hidden=collapse_filters
                />
                {
                    if collapse_filters && (self.props.number_of_filters > 0) && !self.props.is_expanded
                    {
                        html!{
                            <ToolbarGroup>
                                <ToolbarItem>{self.props.number_of_filters}{" filters applied"}</ToolbarItem>
                            </ToolbarGroup>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if self.props.show_clear_filters_button && !self.props.is_expanded
                    {
                        html!{
                            <ToolbarItem>
                                <Button 
                                    variant=ButtonVariant::Link
                                    is_inline=true
                                    onclick=self.link.callback(|_| ToolbarChipGroupContentMsg::ClearChipGroups)
                                >
                                    { &self.props.clear_filters_button_text }
                                </Button>
                            </ToolbarItem>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
            </div>
        }
    }
}
