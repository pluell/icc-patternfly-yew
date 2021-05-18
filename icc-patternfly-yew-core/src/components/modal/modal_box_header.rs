use yew::{
    prelude::*,
};


pub struct ModalBoxHeader
{
    props: ModalBoxHeaderProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxHeaderProperties
{
    /** Content rendered inside the Header */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the button */
    #[prop_or_default]
    pub class_name: String,
    /** Optional help section for the Modal Header */
    #[prop_or_default]
    pub help: Option<Html>,
}

impl Component for ModalBoxHeader
{
    type Message = ();
    type Properties = ModalBoxHeaderProperties;

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
            <header 
                class=classes!(
                    "pf-c-modal-box__header", 
                    if self.props.help.is_some() { "pf-m-help" } else { "" },
                    self.props.class_name.clone(),
                )
                // {...props}
            >
            {
                if let Some(help) = &self.props.help
                {
                    html!{
                        <>
                            <div class="pf-c-modal-box__header-main">
                                { for self.props.children.iter() }
                            </div>
                            <div class="pf-c-modal-box__header-help">
                                { help.clone() }
                            </div>
                        </>
                    }
                }
                else
                {
                    html!{
                        for self.props.children.iter()
                    }
                }
            }
            </header>
        }
    }
}
