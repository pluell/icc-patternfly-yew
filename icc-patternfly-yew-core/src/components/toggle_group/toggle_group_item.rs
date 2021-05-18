use yew::{
    prelude::*,
};

use super::*;


pub struct ToggleGroupItem
{
    link: ComponentLink<Self>,
    props: ToggleGroupItemProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToggleGroupItemProps
{
    /** Text rendered inside the toggle group item */
    #[prop_or_default]
    pub text: Option<Html>,
    /** Icon rendered inside the toggle group item */
    #[prop_or_default]
    pub icon: Option<Html>,
    /** Additional classes added to the toggle group item */
    #[prop_or_default]
    pub class_name: String,
    /** Flag indicating if the toggle group item is disabled */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag indicating if the toggle group item is selected */
    #[prop_or_default]
    pub is_selected: bool,
    /** required when icon is used with no supporting text */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Optional id for the button within the toggle group item */
    #[prop_or_default]
    pub button_id: String,
    /** A callback for when the toggle group item selection changes. */
    #[prop_or_default]
    pub onchange: Callback<bool>, // (selected: boolean, event: React.MouseEvent<any> | React.KeyboardEvent | MouseEvent) => void;
}

pub enum ToggleGroupItemMsg
{
    OnClick(MouseEvent),
}

impl Component for ToggleGroupItem
{
    type Message = ToggleGroupItemMsg;
    type Properties = ToggleGroupItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            props,
            link,
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
            ToggleGroupItemMsg::OnClick(_event) => {
                self.props.onchange.emit(!self.props.is_selected);
            },
        }

        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div 
                class=classes!(
                    "pf-c-toggle-group__item", 
                    self.props.class_name.clone()
                )
                // {...props}
            >
                <button
                    type="button"
                    class=classes!(
                        "pf-c-toggle-group__button", 
                        if self.props.is_selected { "pf-m-selected" } else { "" },
                    )
                    aria-pressed=self.props.is_selected.to_string()
                    onclick=self.link.callback(ToggleGroupItemMsg::OnClick)
                    aria-label=self.props.aria_label.clone()
                    disabled=self.props.is_disabled
                    id=self.props.button_id.clone()
                >
                {
                    if let Some(icon) = &self.props.icon
                    {
                        html!{
                            <ToggleGroupItemElement variant={ToggleGroupItemVariant::Icon}>{icon.clone()}</ToggleGroupItemElement>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if let Some(text) = &self.props.text
                    {
                        html!{
                            <ToggleGroupItemElement variant={ToggleGroupItemVariant::Text}>{text.clone()}</ToggleGroupItemElement>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                </button>
            </div>
        }
    }
}
