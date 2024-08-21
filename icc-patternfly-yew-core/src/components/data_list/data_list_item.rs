use yew::prelude::*;


pub struct DataListItem;

#[derive(Clone, PartialEq, Properties)]
pub struct DataListItemProps
{
    /** Flag to show if the expanded content of the DataList item is visible */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Content rendered inside the DataList item */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the DataList item should be either <DataListItemRow> or <DataListContent> */
    #[prop_or_default]
    pub class_name: String,
    /** Adds accessible text to the DataList item */
    #[prop_or_default]
    pub aria_labelledby: String,
    /** Unique id for the DataList item */
    #[prop_or_default]
    pub id: Option<String>,
}

impl Component for DataListItem
{
    type Message = ();
    type Properties = DataListItemProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <li
                id={ctx.props().id.clone()}
                class={classes!(
                    "pf-v5-c-data-list__item",
                    if ctx.props().is_expanded { "pf-m-expanded" } else { "" },
                    // isSelectable && styles.modifiers.selectable,
                    // selectedDataListItemId && selectedDataListItemId === id && styles.modifiers.selected,
                    ctx.props().class_name.clone()
                )}
                aria-labelledby={ctx.props().aria_labelledby.clone()}
                // {...(isSelectable && { tabIndex: 0, onClick: selectDataListItem, onKeyDown })}
                // {...(isSelectable && selectedDataListItemId === id && { 'aria-selected': true })}
                // {...props}
                // {...dragProps}
            >
            //   {React.Children.map(
            //     children,
            //     child =>
            //       React.isValidElement(child) &&
            //       React.cloneElement(child as React.ReactElement<any>, {
            //         rowid: ariaLabelledBy
            //       })
            //   )}
                { for ctx.props().children.iter() }
            </li>
        }
    }
}
