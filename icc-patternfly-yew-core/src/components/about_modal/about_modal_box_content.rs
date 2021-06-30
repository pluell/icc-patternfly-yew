use yew::{
    prelude::*,
};


pub struct AboutModalBoxContent
{
    props: AboutModalBoxContentProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalBoxContentProps
{
    /** content rendered inside the AboutModalBoxContent  */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the AboutModalBoxContent  */
    #[prop_or_default]
    pub class_name: String,
    /** id to use for About Modal Box aria described by  */
    pub id: String,
    /** The Trademark info for the product  */
    pub trademark: String,
    /** Prevents the about modal from rendering content inside a container; allows for more flexible layouts */
    #[prop_or_default]
    pub no_about_modal_box_content_container: bool,
}

impl Component for AboutModalBoxContent
{
    type Message = ();
    type Properties = AboutModalBoxContentProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props
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
            <div class=classes!("pf-c-about-modal-box__content", self.props.class_name.clone())
                id=self.props.id.clone()
                // {...props}
            >
                <div class=classes!("pf-c-about-modal-box__body")>
                {
                    if self.props.no_about_modal_box_content_container
                    {
                        html!{
                            for self.props.children.iter()
                        }
                    }
                    else
                    {
                        html!{
                            <div class=classes!("pf-c-content")>{for self.props.children.iter()}</div>
                        }
                    }
                }
                </div>
                <p class=classes!("pf-c-about-modal-box__strapline")>{self.props.trademark.clone()}</p>
            </div>
        }
    }
}
