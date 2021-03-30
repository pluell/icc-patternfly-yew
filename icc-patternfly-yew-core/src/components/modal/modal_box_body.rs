use yew::{
    prelude::*,
};


pub struct ModalBoxBody
{
    props: ModalBoxBodyProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxBodyProperties
{
    /** Content rendered inside the ModalBoxBody */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the ModalBoxBody */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for ModalBoxBody
{
    type Message = ();
    type Properties = ModalBoxBodyProperties;

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
                //{...props} 
                class=(
                    "pf-c-modal-box__body",
                    self.props.class_name.clone(),
                )
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
