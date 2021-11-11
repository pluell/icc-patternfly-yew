use yew::{
    prelude::*,
};

use crate::{Button, ButtonVariant};


pub enum DataListToggleMsg
{
    OnClick,
}

pub struct DataListToggle
{
    link: ComponentLink<Self>,
    props: DataListToggleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DataListToggleProps
{
    /** Additional classes added to the DataList cell */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show if the expanded content of the DataList item is visible */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Identify the DataList toggle number */
    pub id: String,
    /** Id for the row */
    #[prop_or_default]
    pub rowid: String,
    /** Adds accessible text to the DataList toggle */
    #[prop_or_default]
    pub aria_labelledby: String,
    /** Adds accessible text to the DataList toggle */
    #[prop_or_default]
    pub aria_label: String,
    /** Allows users of some screen readers to shift focus to the controlled element. Should be used when the controlled contents are not adjacent to the toggle that controls them. */
    #[prop_or_default]
    pub aria_controls: Option<String>,

    pub onclick: Callback<()>,
}

impl Component for DataListToggle
{
    type Message = DataListToggleMsg;
    type Properties = DataListToggleProps;

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
            DataListToggleMsg::OnClick => {
                self.props.onclick.emit(());
            },
        }

        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div 
                class=classes!(
                    "pf-c-data-list__item-control",
                    self.props.class_name.clone()
                )
                // {...props}
            >
                <div class="pf-c-data-list__toggle">
                    <Button
                        id=self.props.id.clone()
                        variant=ButtonVariant::Plain
                        aria_controls=self.props.aria_controls.clone()
                        aria_label=self.props.aria_label.to_string()
                        aria_labelledby=if self.props.aria_label != "Details" { None } else { Some(format!("{}-{}", self.props.rowid, self.props.id)) }
                        aria_expanded=self.props.is_expanded.to_string()

                        onclick=self.link.callback(|_| DataListToggleMsg::OnClick)
                    >
                        <div class="pf-c-data-list__toggle-icon">
                            {icc_patternfly_yew_icons::angle_right_icon!{}}
                        </div>
                    </Button>
                </div>
            </div>
        }
    }
}
