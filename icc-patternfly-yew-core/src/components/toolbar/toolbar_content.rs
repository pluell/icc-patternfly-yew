use yew::{
    prelude::*,
    html::ChildrenRenderer,
};

use super::*;


pub struct ToolbarContent;

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarContentProperties
{
    /** Classes applied to root element of the data toolbar content row */
    #[prop_or_default]
    pub classes: Classes,
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
}

impl Component for ToolbarContent
{
    type Message = ();
    type Properties = ToolbarContentProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {        
        html!{
            <div
                class={classes!(
                    "pf-v5-c-toolbar__content",
                    ctx.props().classes.clone(),
                )}
            >
                <div class="pf-v5-c-toolbar__content-section">
                {
                    for ctx.props().children.iter()
                }
                </div>
            </div>
        }
    }
}
