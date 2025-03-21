use yew::prelude::*;

use crate::styles::{WrapModifers};


pub struct DataListItemRow;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-data-list__item-row",
                    ctx.props().class_name.clone(),
                    if let Some(wrap_modifier) = &ctx.props().wrap_modifier { wrap_modifier.get_class() } else { "" },
                )}
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
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
