use yew::{
    prelude::*,
};


pub struct DataListItem
{
    props: DataListItemProps,
}

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
            <li
                id=self.props.id.clone()
                class=classes!(
                    "pf-c-data-list__item",
                    if self.props.is_expanded { "pf-m-expanded" } else { "" },
                    // isSelectable && styles.modifiers.selectable,
                    // selectedDataListItemId && selectedDataListItemId === id && styles.modifiers.selected,
                    self.props.class_name.clone()
                )
                aria-labelledby=self.props.aria_labelledby.clone()
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
                { for self.props.children.iter() }
            </li>
        }
    }
}
