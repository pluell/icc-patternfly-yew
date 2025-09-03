use yew::prelude::*;

use crate::ToggleGroupItem;


pub struct ToggleGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct ToggleGroupProps
{
    /** Content rendered inside the toggle group */
    #[prop_or_default]
    pub children: ChildrenWithProps<ToggleGroupItem>,
    /** Additional classes added to the toggle group */
    #[prop_or_default]
    pub classes: Classes,
    /** Modifies the toggle group to include compact styling. */
    #[prop_or_default]
    pub is_compact: bool,
    /** Disable all toggle group items under this component. */
    #[prop_or_default]
    pub are_all_groups_disabled: bool,
    /** Accessible label for the toggle group */
    #[prop_or_default]
    pub aria_label: AttrValue,
}

impl Component for ToggleGroup
{
    type Message = ();
    type Properties = ToggleGroupProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-toggle-group", 
                    if ctx.props().is_compact { "pf-m-compact" } else { "" },
                    ctx.props().classes.clone()
                )}
                role="group"
                aria-label={ctx.props().aria_label.clone()}
                // {...props}
            >
            { 
                for ctx.props().children.iter().map(|mut group_item| {
                    let props = std::rc::Rc::make_mut(&mut group_item.props);

                    // Mark the children as disabled if all groups are disabled
                    if ctx.props().are_all_groups_disabled {
                        props.is_disabled = true;
                    }

                    group_item
                }) 
            }
            </div>
        }
    }
}
