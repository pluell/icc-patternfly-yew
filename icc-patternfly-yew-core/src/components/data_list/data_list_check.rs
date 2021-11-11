use yew::{
    prelude::*,
};


pub enum DataListCheckMsg
{
    OnClick,
}

pub struct DataListCheck
{
    link: ComponentLink<Self>,
    props: DataListCheckProps,
}

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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            link,
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
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            DataListCheckMsg::OnClick => {
                if let Some(onchange) = &self.props.onchange
                {
                    onchange.emit(self.props.is_checked);
                }
            },
        }

        false
    }

    fn view(&self) -> Html
    {
        let check = html!{
            <div class="pf-c-data-list__check">
                <input
                    // {...props}
                    type="checkbox"
                    onclick=self.link.callback(|_| DataListCheckMsg::OnClick)
                    aria-invalid=(!self.props.is_valid).to_string()
                    disabled=self.props.is_disabled
                    checked=self.props.is_checked
                />
            </div>
        };

        if !self.props.other_controls
        {
            html!{
                <div
                    class=classes!(
                        "pf-c-data-list__item-control",
                        self.props.class_name.clone()
                    )
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
