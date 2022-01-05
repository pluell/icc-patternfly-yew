use yew::{
    prelude::*,
};


#[derive(Clone, PartialEq)]
pub struct FormFiledGroupHeaderTitleTextObject
{
    /** Title text. */
    pub text: Html,
    /** The applied to the title div for accessibility */
    pub id: String,
}

pub struct FormFieldGroupHeader
{
    props: FormFieldGroupHeaderProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormFieldGroupHeaderProps
{
    /** Additional classes added to the section */
    #[prop_or_default]
    pub class_name: String,
    /** Field group header title text */
    #[prop_or_default]
    pub title_text: Option<FormFiledGroupHeaderTitleTextObject>,
    /** Field group header title description */
    #[prop_or_default]
    pub title_description: Option<Html>,
    /** Field group header actions */
    #[prop_or_default]
    pub actions: Option<Html>,
}

impl Component for FormFieldGroupHeader
{
    type Message = ();
    type Properties = FormFieldGroupHeaderProps;

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
                class=classes!(
                    "pf-c-form__field-group-header",
                    self.props.class_name.clone(),
                )
                // {...props}
            >
                <div class="pf-c-form__field-group-header-main">
                {
                    if let Some(title_text) = &self.props.title_text
                    {
                        html!{
                            <div class="pf-c-form__field-group-header-title">
                                <div class="pf-c-form__field-group-header-title-text" id=title_text.id.clone()>
                                    {title_text.text.clone()}
                                </div>
                            </div>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if let Some(title_description) = &self.props.title_description
                    {
                        html!{
                            <div class="pf-c-form__field-group-header-description">{title_description.clone()}</div>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                </div>
                <div class="pf-c-form__field-group-header-actions">
                {
                    if let Some(actions) = &self.props.actions
                    {
                        actions.clone()
                    }
                    else
                    {
                        html!{}
                    }
                }
                </div>
            </div>
        }
    }
}
