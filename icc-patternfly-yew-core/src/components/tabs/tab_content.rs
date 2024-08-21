use yew::{
    prelude::*,
    virtual_dom::VChild,
};

use super::Tab;


pub struct TabContent;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let has_children = ctx.props().children.len() > 0;

        let is_hidden = if has_children {
                false
            }
            else if let Some(child) = &ctx.props().child
            {
                child.props.event_key != ctx.props().active_key
            }
            else
            {
                false
            };

        html!{
            <section
                // ref={innerRef}
                // hidden={if has_children { false } else { child.props.eventKey != ctx.props().active_key } }
                hidden={is_hidden}
                class={classes!("pf-v5-c-tab-content")}
                // children
                //     ? css('', className, variantStyle[variant])
                //     : css('pf-v5-c-tab-content', child.props.className, variantStyle[variant])
                id={if has_children { ctx.props().id.to_string() } else { format!("pf-tab-section-{}-{}", ctx.props().event_key, ctx.props().id) } }
                aria-label={ctx.props().aria_label.clone()}
                // aria-labelledby={labelledBy}
                role="tabpanel"
                tabIndex={0}
                // {...getOUIAProps('TabContent', ouiaId, ouiaSafe)}
                // {...props}
            >
            {
                if has_children
                {
                    html!{
                        <>{ ctx.props().children.clone() }</>
                    }
                }
                else if let Some(child) = &ctx.props().child
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