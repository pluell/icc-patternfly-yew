use yew::{
    prelude::*,
};
use web_sys::{Element, Event};
use gloo::events::{EventListener, EventListenerOptions};

use crate::helpers::{is_element_in_view};

use super::*;

#[derive(Clone, PartialEq)]
pub enum TabsComponent
{
    Div,
    Nav,
}

#[derive(Clone, PartialEq)]
pub enum TabsStyleVariant
{
    Default,
    Light300,
}

const TABS_VARIANT_STYLES: &'static [&'static str] = &[
    "",
    "pf-m-color-scheme--light-300",
];

pub struct Tabs
{
    link: ComponentLink<Self>,
    props: TabsProperties,
    tab_list_ref: NodeRef,
    _resize_listener_handle: Option<EventListener>,
    show_scroll_buttons: bool,
    disable_left_scroll_button: bool,
    disable_right_scroll_button: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabsProperties
{
    /** Content rendered inside the tabs component. Must be React.ReactElement<TabProps>[] */
    pub children: ChildrenWithProps<Tab>,
    /** Additional classes added to the tabs */
    #[prop_or_default]
    pub class_name: String,
    /** Tabs background color variant */
    #[prop_or(TabsStyleVariant::Default)]
    pub variant: TabsStyleVariant,
    /** The index of the active tab */
    #[prop_or_default]
    pub active_key: String,    //number | string;
    /** Callback to handle tab selection */
    #[prop_or_default]
    pub onselect: Callback<String>,
    /** Uniquely identifies the tabs */
    #[prop_or_default]
    pub id: String,
    /** Enables the filled tab list layout */
    #[prop_or_default]
    pub is_filled: bool,
    /** Enables secondary tab styling */
    #[prop_or_default]
    pub is_secondary: bool,
    /** Enables box styling to the tab component */
    #[prop_or_default]
    pub is_box: bool,
    /** Enables vertical tab styling */
    #[prop_or_default]
    pub is_vertical: bool,
    /** Aria-label for the left scroll button */
    #[prop_or(String::from("Scroll left"))]
    pub left_scroll_aria_label: String,
    /** Aria-label for the right scroll button */
    #[prop_or(String::from("Scroll right"))]
    pub right_scroll_aria_label: String,
    /** Determines what tag is used around the tabs. Use "nav" to define the tabs inside a navigation region */
    #[prop_or(TabsComponent::Div)]
    pub component: TabsComponent,
    /** Provides an accessible label for the tabs. Labels should be unique for each set of tabs that are present on a page. When component is set to nav, this prop should be defined to differentiate the tabs from other navigation regions on the page. */
    #[prop_or_default]
    aria_label: String,
    /** Waits until the first "enter" transition to mount tab children (add them to the DOM) */
    #[prop_or_default]
    pub mount_on_enter: bool,
    /** Unmounts tab children (removes them from the DOM) when they are no longer visible */
    #[prop_or_default]
    pub unmount_on_exit: bool,
    // /** Insets at various breakpoints. */
    // inset?: {
    //     default?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     sm?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     md?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     lg?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     xl?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     '2xl'?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    // };
}

pub enum TabsMsg
{
    OnClickTab(String),
    HandleScrollButtons,
    ScrollLeft,
    ScrollRight,
}

impl Component for Tabs
{
    type Message = TabsMsg;
    type Properties = TabsProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let mut resize_listener_handle = None;

        // Handle scrollbars if we are not using the vertical style of tabs
        if !props.is_vertical
        {
            let window = web_sys::window()
                    .expect("no global `window` exists");
        
            let callback = link.callback(|_| TabsMsg::HandleScrollButtons);

            let listener = move |_: &Event| {
                // Update scrollbars
                callback.emit(());
            };

            let event_options = EventListenerOptions::enable_prevent_default();
            
            // Listen for resize on the whole window to handle the scrollbars
            // when the user resizes the window
            resize_listener_handle = Some(EventListener::new_with_options(
                window.as_ref(),
                "resize",
                event_options,
                listener,
            ));

            link.send_message(TabsMsg::HandleScrollButtons);
        }

        Self {
            link,
            props,
            tab_list_ref: NodeRef::default(),
            _resize_listener_handle: resize_listener_handle,
            show_scroll_buttons: false,
            disable_left_scroll_button: false,
            disable_right_scroll_button: false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;

            // Recalculate scrollbars
            self.link.send_message(TabsMsg::HandleScrollButtons);

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
            TabsMsg::OnClickTab(event_key) => {
                self.props.onselect.emit(event_key.clone());
                
                false
            },
            TabsMsg::HandleScrollButtons => {
                if !self.props.is_vertical
                {
                    if let Some(container) = self.tab_list_ref.cast::<Element>()
                    {
                        // get first element and check if it is in view
                        let overflow_on_left = if let Some(first_child) = container.first_element_child() {
                            !is_element_in_view(&container, &first_child, false)
                        } else {
                            false
                        };
            
                        // get last element and check if it is in view
                        let overflow_on_right = if let Some(first_child) = container.last_element_child() {
                            !is_element_in_view(&container, &first_child, false)
                        } else {
                            false
                        };
            
                        self.show_scroll_buttons = overflow_on_left || overflow_on_right;
                
                        self.disable_left_scroll_button = !overflow_on_left;
                        self.disable_right_scroll_button = !overflow_on_right;
                    }
                }

                true
            },
            TabsMsg::ScrollLeft => {
                // find first Element that is fully in view on the left, then scroll to the element before it
                if let Some(container) = self.tab_list_ref.cast::<Element>()
                {
                    let child_arr = container.children();
                    
                    let mut last_element_out_of_view = None;
                    
                    for i in 0..child_arr.length()
                    {
                        if let Some(child_element) = child_arr.get_with_index(i)
                        {
                            if is_element_in_view(&container, &child_element, false)
                            {
                                last_element_out_of_view = child_arr.get_with_index(i - 1);

                                break;
                            }
                        }
                    }
                    
                    if let Some(last_element) = last_element_out_of_view
                    {
                        container.set_scroll_left(container.scroll_left() - last_element.scroll_width());
                    }
                }

                true
            },
            TabsMsg::ScrollRight => {
                // find last Element that is fully in view on the right, then scroll to the element after it
                if let Some(container) = self.tab_list_ref.cast::<Element>()
                {
                    let child_arr = container.children();
                    
                    let mut first_element_out_of_view = None;
                    
                    for i in 0..child_arr.length()
                    {
                        if let Some(child_element) = child_arr.get_with_index(i)
                        {
                            if is_element_in_view(&container, &child_element, false)
                            {
                                first_element_out_of_view = child_arr.get_with_index(i + 1);

                                break;
                            }
                        }
                    }

                    if let Some(first_element) = first_element_out_of_view
                    {
                        container.set_scroll_left(container.scroll_left() + first_element.scroll_width());
                    }
                }

                true
            },
        }
    }

    fn view(&self) -> Html
    {
        match self.props.component
        {
            TabsComponent::Div => {
                html!{
                    <>
                    <div
                        aria-label=self.props.aria_label.clone()
                        class=classes!(
                            "pf-c-tabs",
                            if self.props.is_filled { "pf-m-fill" } else { "" },
                            if self.props.is_secondary { "pf-m-secondary" } else { "" },
                            if self.props.is_vertical { "pf-m-vertical" } else { "" },
                            if self.props.is_box { "pf-m-box" } else { "" },
                            if self.show_scroll_buttons && !self.props.is_vertical { "pf-m-scrollable" } else {""},
                            // formatBreakpointMods(inset, styles),
                            TABS_VARIANT_STYLES[self.props.variant.clone() as usize],
                            self.props.class_name.to_string(),
                        )
                        // {...getOUIAProps(Tabs.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId, ouiaSafe)}
                        id=self.props.id.clone()
                        // {...props}
                    >
                        { self.render_tabs_control() }
                    </div>
                    { self.render_child() }
                    </>
                }
            },
            TabsComponent::Nav => {
                html!{
                    <nav>
                    </nav>
                }
            },
        }
    }
}

impl Tabs
{
    fn render_tabs_control(&self) -> Html
    {
        html!{
            <>
            <button
                class=classes!(
                    "pf-c-tabs__scroll-button", 
                    if self.props.is_secondary { "pf-m-secondary" } else { "" }
                )
                aria-label=self.props.left_scroll_aria_label.clone()
                onclick=self.link.callback(|_| TabsMsg::ScrollLeft)
                disabled=self.disable_left_scroll_button
                aria-hidden=self.disable_left_scroll_button.to_string()
            >
            {
                icc_patternfly_yew_icons::angle_left_icon!{}
            }
            </button>
            <ul class="pf-c-tabs__list"
                ref=self.tab_list_ref.clone()
                onscroll=self.link.callback(|_| TabsMsg::HandleScrollButtons)
            >
                {
                    for self.props.children.iter().enumerate().map(|(index, child)| {
                        html!{
                            <li
                                key=index
                                class=classes!(
                                    "pf-c-tabs__item", 
                                    if child.props.event_key == self.props.active_key { "pf-m-current" } else { "" },
                                    child.props.class_name.to_string(),
                                )
                            >
                                <TabButton
                                    class_name="pf-c-tabs__link"
                                    onclick=self.link.callback(|event_key| TabsMsg::OnClickTab(event_key))
                                    id=format!("pf-tab-{}-{}", child.props.event_key, self.props.id)   // {`pf-tab-${eventKey}-${childId || uniqueId}`}
                    //               aria-controls={ariaControls}
                    //               tabContentRef={tabContentRef}
                    //               ouiaId={childOuiaId}
                    //               {...rest}
                                    event_key=child.props.event_key.clone()
                                >
                                    {child.props.title}
                                </TabButton>
                            </li>
                        }
                    })
                }
            </ul>
            <button
                class=classes!(
                    "pf-c-tabs__scroll-button", 
                    if self.props.is_secondary { "pf-m-secondary" } else { "" }
                )
                aria-label=self.props.right_scroll_aria_label.clone()
                onclick=self.link.callback(|_| TabsMsg::ScrollRight)
                disabled=self.disable_right_scroll_button
                aria-hidden=self.disable_right_scroll_button.to_string()
            >
            {
                icc_patternfly_yew_icons::angle_right_icon!{}
            }
            </button>
            </>
        }
    }

    fn render_child(&self) -> Html
    {
        self.props.children.iter().map(|tab| {
            html!{
                <TabContent
                    // key={index}
                    active_key=self.props.active_key.to_string()
                    child=tab.clone()
                    // id=tab.props.id.to_string() //{child.props.id || uniqueId}
                    // ouiaId={child.props.ouiaId}
                />
            }
        })
        .collect::<Html>()
    }
}