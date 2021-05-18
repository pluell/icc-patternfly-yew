use yew::{
    prelude::*,
    virtual_dom::VChild,
};

use super::*;

pub struct Alert
{
    props: AlertProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AlertProps
{
    /** Adds Alert variant styles  */
    #[prop_or(AlertVariant::Default)]
    pub variant: AlertVariant,
    /** Flag to indicate if the Alert is inline */
    #[prop_or_default]
    pub is_inline: bool,
    /** Title of the Alert  */
    pub title: String,
    // /** Close button; use the AlertActionCloseButton component  */
    #[prop_or_default]
    pub action_close: Option<VChild<AlertActionCloseButton>>,
    /** Action links; use a single AlertActionLink component or multiple wrapped in an array or React.Fragment */
    #[prop_or_default]
    pub action_links: Option<Vec<VChild<AlertActionLink>>>,
    /** Content rendered inside the Alert */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Alert  */
    #[prop_or_default]
    pub class_name: String,
    /** Adds accessible text to the Alert */
    #[prop_or_default]
    pub aria_label: String,
    // /** Variant label text for screen readers */
    #[prop_or_default]
    pub variant_label: String,
    // /** Flag to indicate if the Alert is in a live region */
    #[prop_or_default]
    pub is_live_region: bool,
    // /** If set to true, the timeout is 8000 milliseconds. If a number is provided, alert will be dismissed after that amount of time in milliseconds. */
    // timeout?: number | boolean;
    // /** If the user hovers over the Alert and `timeout` expires, this is how long to wait before finally dismissing the Alert */
    // timeoutAnimation?: number;
    // /** Function to be executed on alert timeout. Relevant when the timeout prop is set */
    // onTimeout?: () => void;
    /** Truncate title to number of lines */
    #[prop_or_default]
    pub truncate_title: i32,
    // /** Position of the tooltip which is displayed if text is truncated */
    // tooltipPosition?: 'auto' | 'top' | 'bottom' | 'left' | 'right';
    /** Set a custom icon to the Alert. If not set the icon is set according to the variant */
    #[prop_or_default]
    pub custom_icon: Option<Html>,
}

impl Component for Alert
{
    type Message = ();
    type Properties = AlertProps;

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
                // ref={divRef}
                class=classes!(
                    "pf-c-alert",
                    if self.props.is_inline { "pf-m-inline" } else { "" },
                    self.props.variant.class(),
                    &self.props.class_name
                )
                aria-label=self.props.aria_label.clone()
                // {...ouiaProps}
                // {...(isLiveRegion && {
                //     'aria-live': 'polite',
                //     'aria-atomic': 'false'
                // })}
                // onMouseEnter={myOnMouseEnter}
                // onMouseLeave={myOnMouseLeave}
                // {...props}
            >
                <AlertIcon variant=self.props.variant.clone() custom_icon=self.props.custom_icon.clone() />
                // if is_tooltip_visible
                // {
                //     <Tooltip content={getHeadingContent} position={tooltipPosition}>
                //         {Title}
                //     </Tooltip>
                // }
                // else
                {
                    self.view_title()
                }
                {
                    if let Some(action_close) = &self.props.action_close
                    {
                        let mut action_button = action_close.clone();

                        // Pass through the title for context
                        action_button.props.title = self.props.title.clone();

                        html!{
                            <div class="pf-c-alert__action">
                            {
                                action_button
                            }
                            </div>
                        }
                        
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    html!{
                        <div class="pf-c-alert__description">
                        {
                            for self.props.children.iter()
                        }
                        </div>
                    }

                }
                {
                    if let Some(action_links) = &self.props.action_links
                    {
                        html!{
                            <div class="pf-c-alert__action-group">
                            {
                                action_links.clone()
                            }
                            </div>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
            </div>
        }
    }
}

impl Alert
{
    pub fn view_title(&self) -> Html
    {
        let variant_label = if self.props.variant_label.len() > 0 {
            self.props.variant_label.clone()
        } else {
            format!("{} alert:", self.props.variant)
        };

        html!{
            <h4
                // {...(isTooltipVisible && { tabIndex: 0 })}
                // ref={titleRef}
                class=classes!(
                    "pf-c-alert__title", 
                    if self.props.truncate_title > 0 { "pf-m-truncate" } else { "" },
                )
            >
                <span class="pf-screen-reader">{&variant_label}</span>
                {&self.props.title}
            </h4>
        }
    }
}
