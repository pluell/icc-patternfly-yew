use yew::{
    prelude::*,
};


pub struct SelectToggle
{
    link: ComponentLink<Self>,
    props: SelectToggleProperties,
}

pub enum SelectToggleMsg
{
    OnToggle,
    OnKeyDown(KeyboardEvent),
}

#[derive(Clone, PartialEq, Properties)]
pub struct SelectToggleProperties
{
    pub is_disabled: bool,
    pub is_open: bool,
    pub ontoggle: Callback<bool>,
    pub children: Children,
}

impl Component for SelectToggle
{
    type Message = SelectToggleMsg;
    type Properties = SelectToggleProperties;

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
            SelectToggleMsg::OnToggle => {
                self.props.ontoggle.emit(!self.props.is_open);

                false
            },
            SelectToggleMsg::OnKeyDown(event) => {
                if event.key() == "Enter"
                {
                    self.props.ontoggle.emit(!self.props.is_open);
                }

                false
            },
        }
    }

    fn view(&self) -> Html
    {
        html!{
            <button 
                class="pf-c-select__toggle" 
                type="button" 
                id="select-single-toggle" 
                aria-haspopup="true" aria-expanded="false" aria-labelledby="select-single-label select-single-toggle"
                disabled=self.props.is_disabled
                onclick=self.link.callback(|_| SelectToggleMsg::OnToggle)
                onkeydown=self.link.callback(|event| SelectToggleMsg::OnKeyDown(event))
            >
                { self.props.children.clone() }
                <span class="pf-c-select__toggle-arrow">
                    <i class="fas fa-caret-down" aria-hidden="true"></i>
                </span>
            </button>
        }
    }
}
