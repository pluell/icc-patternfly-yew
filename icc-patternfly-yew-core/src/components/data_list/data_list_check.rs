use yew::prelude::*;


pub enum DataListCheckMsg
{
    OnClick,
}

pub struct DataListCheck;

#[derive(Clone, PartialEq, Properties)]
pub struct DataListCheckProps
{
    /** Additional classes added to the DataList item checkbox */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show if the DataList checkbox selection is valid or invalid */
    #[prop_or(true)]
    pub is_valid: bool,
    /** Flag to show if the DataList checkbox is disabled */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag to show if the DataList checkbox is checked */
    #[prop_or_default]
    pub is_checked: bool,
    /** Alternate Flag to show if the DataList checkbox is checked */
    #[prop_or_default]
    pub checked: bool,
    /** A callback for when the DataList checkbox selection changes */
    #[prop_or_default]
    pub onchange: Option<Callback<bool>>,
    /** Aria-labelledby of the DataList checkbox */
    pub aria_labelledby: String,
    /** Flag to indicate if other controls are used in the DataListItem */
    #[prop_or_default]
    pub other_controls: bool,
}

impl Component for DataListCheck
{
    type Message = DataListCheckMsg;
    type Properties = DataListCheckProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            DataListCheckMsg::OnClick => {
                if let Some(onchange) = &ctx.props().onchange
                {
                    onchange.emit(ctx.props().is_checked);
                }
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let check = html!{
            <div class={"pf-v5-c-data-list__check"}>
                <input
                    // {...props}
                    type={"checkbox"}
                    onclick={ctx.link().callback(|_| DataListCheckMsg::OnClick)}
                    aria-invalid={(!ctx.props().is_valid).to_string()}
                    disabled={ctx.props().is_disabled}
                    checked={ctx.props().is_checked}
                />
            </div>
        };

        if !ctx.props().other_controls
        {
            html!{
                <div
                    class={classes!(
                        "pf-v5-c-data-list__item-control",
                        ctx.props().class_name.clone()
                    )}
                >
                    {check}
                </div>
            }
        }
        else
        {
            check
        }
    }
}
