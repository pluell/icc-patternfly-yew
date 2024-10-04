use wasm_bindgen::JsCast;
use yew::prelude::*;

use super::{DrawerContext, DrawerColorVariant, DrawerPosition};


pub struct DrawerPanelContent
{
    context: DrawerContext,
    _context_listener: ContextHandle<DrawerContext>,
    panel_ref: NodeRef,
    splitter_ref: NodeRef,
    hidden: bool,
    is_expanded: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerPanelContentProps
{
    /** Additional classes added to the drawer. */
    #[prop_or_default]
    pub classes: Classes,
    /** ID of the drawer panel */
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /** Content to be rendered in the drawer panel. */
    #[prop_or_default]
    pub children: Html,
    /** Flag indicating that the drawer panel should not have a border. */
    #[prop_or_default]
    pub has_no_border: bool,
    /** Flag indicating that the drawer panel should be resizable. */
    #[prop_or_default]
    pub is_resizable: bool,
    /** Callback for resize end. */
    #[prop_or_default]
    pub onresize: Callback<(i32, String)>, //(event: MouseEvent | TouchEvent | React.KeyboardEvent, width: number, id: string) => void;
    /** The minimum size of a drawer. */
    #[prop_or_default]
    pub min_size: Option<AttrValue>,
    /** The starting size of a drawer. */
    #[prop_or_default]
    pub default_size: Option<AttrValue>,
    /** The maximum size of a drawer. */
    #[prop_or_default]
    pub max_size: Option<AttrValue>,
    /** The increment amount for keyboard drawer resizing. */
    #[prop_or_default]
    pub increment: Option<i32>,
    /** Aria label for the resizable drawer splitter. */
    #[prop_or_default]
    pub resize_aria_label: Option<AttrValue>,
    // /** Width for drawer panel at various breakpoints. Overriden by resizable drawer minSize and defaultSize. */
    // #[prop_or_default]
    // widths?: {
    //     default?: 'width_25' | 'width_33' | 'width_50' | 'width_66' | 'width_75' | 'width_100';
    //     lg?: 'width_25' | 'width_33' | 'width_50' | 'width_66' | 'width_75' | 'width_100';
    //     xl?: 'width_25' | 'width_33' | 'width_50' | 'width_66' | 'width_75' | 'width_100';
    //     '2xl'?: 'width_25' | 'width_33' | 'width_50' | 'width_66' | 'width_75' | 'width_100';
    // }
    /** Color variant of the background of the drawer panel */
    #[prop_or(DrawerColorVariant::Default)]
    pub color_variant: DrawerColorVariant,
    // /** Adds and customizes a focus trap on the drawer panel content. */
    // #[prop_or_default]
    // focusTrap?: DrawerPanelFocusTrapObject;
}

pub enum DrawerPanelMessage
{
    Context(DrawerContext),
    OnTransitionEnd(TransitionEvent),
}

impl Component for DrawerPanelContent
{
    type Message = DrawerPanelMessage;
    type Properties = DrawerPanelContentProps;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (context, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::Context))
            .expect("No Drawer Context Provided");

        let hidden = if context.is_static {false} else {!context.is_expanded};

        Self {
            context,
            _context_listener,
            panel_ref: NodeRef::default(),
            splitter_ref: NodeRef::default(),
            hidden,
            is_expanded: !hidden,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg : Self::Message) -> bool
    {
        match msg
        {
            Self::Message::Context(context) => {
                self.context = context;

                // Check if the context values changed and update the is_exanded property
                if !self.context.is_static && self.context.is_expanded
                {
                    self.is_expanded = self.context.is_expanded
                }
                true
            }
            Self::Message::OnTransitionEnd(event) => {
                let mut render = false;
                if let Some(event_node) = event.target().unwrap().dyn_ref::<web_sys::Node>()
                {
                    if let Some(panel_ref) = self.panel_ref.get()
                    {
                        // Make sure the event applies to the drawer panel
                        if &panel_ref == event_node
                        {
                            if !self.hidden && event.property_name() == "transform"
                            {
                                self.context.onexpand.emit(());
                            }

                            self.is_expanded = !self.hidden;

                            // TODO: Implement focus trap
                            // if (isValidFocusTrap && ev.nativeEvent.propertyName === 'transform') {
                            //     setIsFocusTrapActive((prevIsFocusTrapActive) => !prevIsFocusTrapActive);
                            // }

                            render = true;
                        }
                    }
                }

                render
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                // {...(isValidFocusTrap && focusTrapProps)}
                id={ctx.props().id.clone()} //{id || panelId}
                class={classes!(
                    "pf-v5-c-drawer__panel",
                    if ctx.props().is_resizable {"pf-m-resizable"} else {""},
                    if ctx.props().has_no_border {"pf-m-no-borders"} else {""},
                    // formatBreakpointMods(widths, styles),
                    ctx.props().color_variant.get_class(),
                    ctx.props().classes.clone()
                )}
                ontransitionend={ctx.link().callback(DrawerPanelMessage::OnTransitionEnd)}
                hidden={self.hidden}
                // {...((defaultSize || minSize || maxSize) && {
                //     style: boundaryCssVars as React.CSSProperties
                // })}
                // {...props}
                ref={self.panel_ref.clone()}
            >
            {
                html!{
                    if self.is_expanded {
                        <>
                        {
                            if ctx.props().is_resizable {
                                self.view_resizable(ctx)
                            } else {
                                ctx.props().children.clone()
                            }
                        }
                        </>
                    }
                }
            }
            </div>
        }
    }
}

impl DrawerPanelContent
{
    fn view_resizable(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <>
                <div
                    class={classes!(
                        "pf-v5-c-drawer__splitter",
                        if self.context.position != DrawerPosition::Bottom {"pf-m-vertical"} else {""}
                    )}
                    role="separator"
                    tabindex={0}
                    aria-orientation={if self.context.position == DrawerPosition::Bottom {"horizontal"} else {"vertical"}}
                    aria-label={ctx.props().resize_aria_label.clone()}
                    // aria-valuenow={separatorValue}
                    aria-valuemin=0
                    aria-valuemax=100
                    // aria-controls={id || panelId}
                    // onMouseDown={handleMousedown}
                    // onKeyDown={handleKeys}
                    // onTouchStart={handleTouchStart}
                    ref={self.splitter_ref.clone()}
                >
                    <div class="pf-v5-c-drawer__splitter-handle" aria-hidden="true"></div>
                </div>
                <div class="pf-v5-c-drawer__panel">{ctx.props().children.clone()}</div>
            </>
        }
    }
}