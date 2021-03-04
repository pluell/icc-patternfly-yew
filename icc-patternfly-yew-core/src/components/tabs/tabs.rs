use yew::{
    prelude::*,
};

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
    pub onselect: Callback<String>, //(event: React.MouseEvent<HTMLElement, MouseEvent>, eventKey: number | string) => void;
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
    // leftScrollAriaLabel?: string;
    /** Aria-label for the right scroll button */
    // rightScrollAriaLabel?: string;
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
    ScrollLeft,
    ScrollRight,
}

impl Component for Tabs
{
    type Message = TabsMsg;
    type Properties = TabsProperties;

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
            TabsMsg::OnClickTab(event_key) => {
                self.props.onselect.emit(event_key.clone());
                
                false
            },
            TabsMsg::ScrollLeft => {
                true
            },
            TabsMsg::ScrollRight => {
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
                        aria-label=self.props.aria_label
                        class=(
                            "pf-c-tabs",
                            if self.props.is_filled { "pf-m-fill" } else { "" },
                            if self.props.is_secondary { "pf-m-secondary" } else { "" },
                            if self.props.is_vertical { "pf-m-vertical" } else { "" },
                            if self.props.is_box { "pf-m-box" } else { "" },
                            // showScrollButtons && !isVertical && { "pf-m-scrollable" },
                            // formatBreakpointMods(inset, styles),
                            TABS_VARIANT_STYLES[self.props.variant.clone() as usize],
                            self.props.class_name.to_string(),
                        )
                        // {...getOUIAProps(Tabs.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId, ouiaSafe)}
                        id=self.props.id
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
                class=(
                    "pf-c-tabs__scroll-button", 
                    if self.props.is_secondary { "pf-m-secondary" } else { "" }
                )
                // aria-label={leftScrollAriaLabel}
                onclick=self.link.callback(|_| TabsMsg::ScrollLeft)
                diabled=true    // disabled={disableLeftScrollButton}
                aria-hidden=true    // aria-hidden={disableLeftScrollButton}
            >
                <i class="fas fa-angle-left"></i>
            </button>
            <ul class="pf-c-tabs__list"
                //ref={this.tabList} 
                //onScroll={this.handleScrollButtons}
            >
                {
                    for self.props.children.iter().enumerate().map(|(index, child)| {
                        html!{
                            <li
                                key=index
                                class=(
                                    "pf-c-tabs__item", 
                                    if child.props.event_key == self.props.active_key { "pf-m-current" } else { "" },
                                    child.props.class_name.to_string(),
                                )
                            >
                                <TabButton
                                    class_name="pf-c-tabs__link"
                                    onclick=self.link.callback(|event_key| TabsMsg::OnClickTab(event_key))   //{(event: any) => this.handleTabClick(event, eventKey, tabContentRef, mountOnEnter)}
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
                class=(
                    "pf-c-tabs__scroll-button", 
                    if self.props.is_secondary { "pf-m-secondary" } else { "" }
                )
                // aria-label={rightScrollAriaLabel}
                onclick=self.link.callback(|_| TabsMsg::ScrollRight)
                diabled=true      //   disabled={disableRightScrollButton}
                aria-hidden=true  //   aria-hidden={disableRightScrollButton}
            >
                <i class="fas fa-angle-left"></i>
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