use yew::{
    prelude::*,
    virtual_dom::{VChild},
};

use super::{Tab};


pub struct TabContent
{
    props: TabContentProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabContentProperties
{
    /** content rendered inside the tab content area if used outside Tabs component */
    #[prop_or_default]
    pub children: Children,
    /** Child to show in the content area */
    #[prop_or_default]
    pub child: Option<VChild<Tab>>, // Option<Html>,    //React.ReactElement<any>;
    /** class of tab content area if used outside Tabs component */
    #[prop_or_default]
    pub class_name: String,
    /** Identifies the active Tab  */
    #[prop_or_default]
    pub active_key: String, // number | string;
    /** uniquely identifies the controlling Tab if used outside Tabs component */
    #[prop_or_default]
    pub event_key: String,  // number | string;
    // /** Callback for the section ref */
    // #[prop_or_default]
    // innerRef?: React.Ref<any>;
    /** id passed from parent to identify the content section */
    #[prop_or_default]
    pub id: String,
    /** title of controlling Tab if used outside Tabs component */
    #[prop_or_default]
    pub aria_label: String,
}

impl Component for TabContent
{
    type Message = ();
    type Properties = TabContentProperties;

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
        let has_children = self.props.children.len() > 0;

        let is_hidden = if has_children {
                false
            }
            else if let Some(child) = &self.props.child
            {
                child.props.event_key != self.props.active_key
            }
            else
            {
                false
            };

        html!{
            <section
                // ref={innerRef}
                // hidden={if has_children { false } else { child.props.eventKey != self.props.active_key } }
                hidden=is_hidden
                class=classes!("pf-c-tab-content")
                // children
                //     ? css('', className, variantStyle[variant])
                //     : css('pf-c-tab-content', child.props.className, variantStyle[variant])
                id={if has_children { self.props.id.to_string() } else { format!("pf-tab-section-{}-{}", self.props.event_key, self.props.id) } }
                aria-label=self.props.aria_label.clone()
                // aria-labelledby={labelledBy}
                role="tabpanel"
                tabIndex=0
                // {...getOUIAProps('TabContent', ouiaId, ouiaSafe)}
                // {...props}
            >
            {
                if has_children
                {
                    html!{
                        <>{ self.props.children.clone() }</>
                    }
                }
                else if let Some(child) = &self.props.child
                {
                    html!{
                        <>{ child.props.children.clone() }</>
                    }
                }
                else
                {
                    html!{}
                }
            }
            </section>
        }
    }
}