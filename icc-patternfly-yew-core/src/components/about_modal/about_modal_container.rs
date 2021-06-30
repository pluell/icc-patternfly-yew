use yew::{
    prelude::*,
};

use crate::{Backdrop};

use super::*;


pub struct AboutModalContainer
{
    props: AboutModalContainerProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalContainerProps
{
    /** content rendered inside the About Modal Box Content.  */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the About Modal Box  */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show the About Modal  */
    #[prop_or_default]
    pub is_open: bool,
    /** A callback for when the close button is clicked  */
    #[prop_or_default]
    pub onclose: Callback<()>,
    /** Product Name  */
    #[prop_or_default]
    pub product_name: Option<String>,
    /** Trademark information  */
    #[prop_or_default]
    pub trademark: String,
    /** the URL of the image for the Brand.  */
    pub brand_image_src: String,
    /** the alternate text of the Brand image.  */
    pub brand_image_alt: String,
    /** the URL of the image for the background.  */
    #[prop_or_default]
    pub background_image_src: Option<String>,
    /** id to use for About Modal Box aria labeled by  */
    pub about_modal_box_header_id: String,
    /** id to use for About Modal Box aria described by  */
    pub about_modal_box_content_id: String,
    /** Set close button aria label */
    #[prop_or_default]
    pub close_button_aria_label: Option<String>,
}

impl Component for AboutModalContainer
{
    type Message = ();
    type Properties = AboutModalContainerProps;

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
        if !self.props.is_open
        {
            html!{}
        }
        else
        {
            html!{
                <Backdrop>
                    // TODO: Convert this to a FocusTrap
                    <div class="pf-l-bullseye">
                        <AboutModalBox
                            class_name=self.props.class_name.clone()
                            aria_labelledby=self.props.about_modal_box_header_id.clone()
                            aria_describedby=self.props.about_modal_box_content_id.clone()
                        >
                            <AboutModalBoxBrand src=self.props.brand_image_src.clone() alt=self.props.brand_image_alt.clone() />
                            <AboutModalBoxCloseButton aria_label=self.props.close_button_aria_label.clone() onclose=self.props.onclose.clone() />
                            {
                                if let Some(product_name) = &self.props.product_name
                                {
                                    html!{
                                        <AboutModalBoxHeader id=self.props.about_modal_box_header_id.clone() product_name=product_name.clone() />
                                    }
                                }
                                else
                                {
                                    html!{}
                                }
                            }
                            <AboutModalBoxContent
                                trademark=self.props.trademark.clone()
                                id=self.props.about_modal_box_content_id.clone()
                                no_about_modal_box_content_container=false
                                // {...props}
                            >
                                {for self.props.children.iter()}
                            </AboutModalBoxContent>
                            <AboutModalBoxHero background_image_src=self.props.background_image_src.clone() />
                        </AboutModalBox>
                    </div>
                </Backdrop>
            }
        }
    }
}
