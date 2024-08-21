use yew::prelude::*;


pub struct DataListControl;

#[derive(Clone, PartialEq, Properties)]
pub struct DataListControlProps
{
    /** Children of the data list control */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the DataList item control */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for DataListControl
{
    type Message = ();
    type Properties = DataListControlProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-data-list__item-control",
                    ctx.props().class_name.clone()
                )}
                // {...props}
            >
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
