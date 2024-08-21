use yew::prelude::*;


pub struct DataListContent;

#[derive(Clone, PartialEq, Properties)]
pub struct DataListContentProps
{
    /** Content rendered inside the DataList item */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the DataList cell */
    #[prop_or_default]
    pub class_name: String,
    /** Identify the DataListContent item */
    #[prop_or_default]
    pub id: Option<String>,
    /** Id for the row */
    #[prop_or_default]
    pub rowid: String,
    /** Flag to show if the expanded content of the DataList item is visible */
    #[prop_or_default]
    pub is_hidden: bool,
    /** Flag to remove padding from the expandable content */
    #[prop_or_default]
    pub has_no_padding: bool,
    /** Adds accessible text to the DataList toggle */
    pub aria_label: String,
}

impl Component for DataListContent
{
    type Message = ();
    type Properties = DataListContentProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <section
                id={ctx.props().id.clone()}
                class={classes!(
                    "pf-v5-c-data-list__expandable-content",
                    ctx.props().class_name.clone()
                )}
                hidden={ctx.props().is_hidden}
                aria-label={ctx.props().aria_label.to_string()}
                // {...props}
            >
                <div 
                    class={classes!(
                        "pf-v5-c-data-list__expandable-content-body", 
                        if ctx.props().has_no_padding { "pf-m-no-padding" } else { "" }
                    )}
                >
                    { for ctx.props().children.iter() }
                </div>
            </section>
        }
    }
}
