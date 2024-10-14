use yew::prelude::*;


#[derive(Clone, PartialEq)]
pub enum DrawerColorVariant 
{
    Default,
    Light200,
    NoBackground,
}

impl DrawerColorVariant
{
    pub fn get_class(&self) -> &'static str
    {
        match self
        {
            Self::Default => "",
            Self::Light200 => "pf-m-light-200",
            Self::NoBackground => "pf-m-no-background",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DrawerContext
{
    pub is_expanded: bool,
    pub is_static: bool, 
    pub onexpand: Callback<()>,
    pub position: DrawerPosition,
    pub drawer_ref: NodeRef,
    pub drawer_content_ref: NodeRef,
    pub is_inline: bool,
}

#[derive(Clone, PartialEq)]
pub enum DrawerPosition
{
    Start,
    End,
    Bottom,
    Left,
    Right,
}

pub struct Drawer
{
    drawer_ref: NodeRef,
    drawer_content_ref: NodeRef,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerProps
{
    /** Additional classes added to the Drawer. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content rendered in the drawer panel */
    #[prop_or_default]
    pub children: Html,
    /** Indicates if the drawer is expanded */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Indicates if the content element and panel element are displayed side by side. */
    #[prop_or_default]
    pub is_inline: bool,
    /** Indicates if the drawer will always show both content and panel. */
    #[prop_or_default]
    pub is_static: bool,
    // /** Position of the drawer panel. left and right are deprecated, use start and end instead. */
    #[prop_or(DrawerPosition::End)]
    pub position: DrawerPosition, // 'start' | 'end' | 'bottom' | 'left' | 'right';
    /** Callback when drawer panel is expanded after waiting 250ms for animation to complete. */
    #[prop_or_default]
    pub onexpand: Callback<()>, //(event: KeyboardEvent | React.MouseEvent | React.TransitionEvent) => void;
}

impl Component for Drawer
{
    type Message = ();
    type Properties = DrawerProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self {
            drawer_ref: NodeRef::default(),
            drawer_content_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let position_cls = match ctx.props().position {
            DrawerPosition::Left | DrawerPosition::Start => "pf-m-panel-left",
            DrawerPosition::Bottom => "pf-m-panel-bottom",
            _ => ""
        };

        html!{
            <ContextProvider<DrawerContext> 
                context={DrawerContext{
                    is_expanded: ctx.props().is_expanded, 
                    is_static: ctx.props().is_static,
                    onexpand: ctx.props().onexpand.clone(),
                    position: ctx.props().position.clone(),
                    drawer_ref: self.drawer_ref.clone(), 
                    drawer_content_ref: self.drawer_content_ref.clone(),
                    is_inline: ctx.props().is_inline,
                }}
            >
                <div
                    class={classes!(
                        "pf-v5-c-drawer",
                        if ctx.props().is_expanded {"pf-m-expanded"} else {""}, // isExpanded && styles.modifiers.expanded,
                        if ctx.props().is_inline {"pf-m-inline"} else {""}, //isInline && styles.modifiers.inline,
                        if ctx.props().is_static {"pf-m-static"} else {""}, // isStatic && styles.modifiers.static,
                        position_cls,
                        ctx.props().classes.clone()
                    )}  
                    ref={self.drawer_ref.clone()}
                    // {...props}
                >
                    {ctx.props().children.clone()}
                </div>
            </ContextProvider<DrawerContext>>
        }
    }
}
