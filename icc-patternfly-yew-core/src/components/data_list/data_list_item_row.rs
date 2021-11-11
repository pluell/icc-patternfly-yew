use yew::{
    prelude::*,
};

use crate::styles::{WrapModifers};


pub struct DataListItemRow
{
    props: DataListItemRowProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DataListItemRowProps
{
    /** Content rendered inside the DataListItemRow  */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the DataListItemRow */
    #[prop_or_default]
    pub class_name: String,
    /** Id for the row item */
    #[prop_or_default]
    pub row_id: Option<String>,
    /** Determines which wrapping modifier to apply to the DataListItemRow */
    #[prop_or_default]
    pub wrap_modifier: Option<WrapModifers>,
}

impl Component for DataListItemRow
{
    type Message = ();
    type Properties = DataListItemRowProps;

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
                class=classes!(
                    "pf-c-data-list__item-row",
                    self.props.class_name.clone(),
                    if let Some(wrap_modifier) = &self.props.wrap_modifier { wrap_modifier.get_class() } else { "" },
                )
                //  {...props}
            >
                // {React.Children.map(
                // children,
                // child =>
                //     React.isValidElement(child) &&
                //     React.cloneElement(child as React.ReactElement<any>, {
                //     rowid
                //     })
                // )}
                { for self.props.children.iter() }
            </div>
        }
    }
}
