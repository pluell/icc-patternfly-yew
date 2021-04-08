use yew::{
    prelude::*,
    html::{ChildrenRenderer},
};

use super::*;


pub struct ToolbarContent
{
    props: ToolbarContentProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarContentProperties
{
    /** Classes applied to root element of the data toolbar content row */
    #[prop_or_default]
    pub class_name: String,
    // /** Visibility at various breakpoints. */
    // visibility?: {
    //     default?: 'hidden' | 'visible';
    //     md?: 'hidden' | 'visible';
    //     lg?: 'hidden' | 'visible';
    //     xl?: 'hidden' | 'visible';
    //     '2xl'?: 'hidden' | 'visible';
    // };
    // /** Alignment at various breakpoints. */
    // alignment?: {
    //     default?: 'alignRight' | 'alignLeft';
    //     md?: 'alignRight' | 'alignLeft';
    //     lg?: 'alignRight' | 'alignLeft';
    //     xl?: 'alignRight' | 'alignLeft';
    //     '2xl'?: 'alignRight' | 'alignLeft';
    // };
    /** Content to be rendered as children of the content row */
    pub children: ChildrenRenderer<ToolbarContentChild>,
    /** Flag indicating if a data toolbar toggle group's expandable content is expanded */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Optional callback for clearing all filters in the toolbar */
    #[prop_or_default]
    pub clear_all_filters: Callback<()>,
    /** Flag indicating that the clear all filters button should be visible */
    #[prop_or_default]
    pub show_clear_filters_button: bool,
    /** Text to display in the clear all filters button */
    #[prop_or_default]
    pub clear_filters_button_text: String,
    /** Id of the parent Toolbar component */
    #[prop_or_default]
    pub toolbar_id: String,
    /** Chip group content reference for passing to data toolbar children */
    #[prop_or_default]
    pub chip_group_content_ref: NodeRef,
    #[prop_or_default]
    pub update_number_filters: Callback<(String, i32)>,
}

impl Component for ToolbarContent
{
    type Message = ();
    type Properties = ToolbarContentProperties;

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
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div
                class=(
                    "pf-c-toolbar__content",
                    &self.props.class_name,
                )
            >
                <div class="pf-c-toolbar__content-section">
                {
                    for self.props.children.iter().map(|mut child| {
                        match child.props
                        {
                            ToolbarContentTypes::Group(ref mut props) => {
                                props.chip_group_content_ref = Some(self.props.chip_group_content_ref.clone());
                                props.update_number_filters = self.props.update_number_filters.clone();
                            },
                            _ => {}
                        }
                        
                        child
                    })
                }
                </div>
            </div>
        }
    }
}
