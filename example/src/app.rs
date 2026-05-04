use yew::prelude::*;
use yew_router::{Switch, prelude::*};

use icc_patternfly_yew_core::{
    AboutModal, Button, ButtonVariant, Masthead, MastheadContent, MastheadMain, MastheadToggle,
    Nav, NavGroup, NavItem, Page, PageSidebar, PageSidebarBody, PageToggleButton, Title,
    TitleHeadingLevels, Toolbar, ToolbarContent, ToolbarGroup, ToolbarItem,
};

use crate::components::*;

pub struct App {
    _location_listener: Option<LocationHandle>,
    cur_app_route: AppRoute,
    about_dialog_is_open: bool,
}

pub enum Msg {
    LocationChanged(Location),
    ToggleAboutDialog,
}

#[derive(Routable, Clone, PartialEq)]
pub enum AppRoute {
    #[not_found]
    #[at("/")]
    Dashboard,

    #[at("/table")]
    TableDemo,

    #[at("/form")]
    FormDemo,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let _location_listener = ctx
            .link()
            .add_location_listener(ctx.link().callback(Msg::LocationChanged));
        let cur_app_route = ctx.link().route::<AppRoute>().unwrap_or_default();

        App {
            cur_app_route,
            _location_listener,
            about_dialog_is_open: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::LocationChanged(_location) => {
                self.cur_app_route = ctx.link().route::<AppRoute>().unwrap_or_default();
            }
            Self::Message::ToggleAboutDialog => {
                self.about_dialog_is_open = !self.about_dialog_is_open;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Page
                    is_managed_sidebar=true
                    header={html!{
                        <Masthead>
                            <MastheadToggle>
                                <PageToggleButton>{icc_patternfly_yew_icons::bars_icon!{}}</PageToggleButton>
                            </MastheadToggle>
                            <MastheadMain>
                                <Title heading_level={TitleHeadingLevels::H1}>{"Yew Patteryfly Demo"}</Title>
                            </MastheadMain>
                            <MastheadContent>
                                <Toolbar>
                                    <ToolbarContent>
                                        <ToolbarGroup classes={"pf-m-align-right"}>
                                            <ToolbarItem>
                                                <Button variant={ButtonVariant::Plain} onclick={ctx.link().callback(|_| Msg::ToggleAboutDialog)}>{icc_patternfly_yew_icons::question_circle_icon!{}}</Button>
                                            </ToolbarItem>
                                        </ToolbarGroup>
                                    </ToolbarContent>
                                </Toolbar>
                            </MastheadContent>
                        </Masthead>
                    }}
                    sidebar={html!{
                        <PageSidebar>
                            <PageSidebarBody>
                                <Nav>
                                    <NavItem is_active={self.cur_app_route == AppRoute::Dashboard}>
                                        <Link<AppRoute> to={AppRoute::Dashboard}>{"Dashboard"}</Link<AppRoute>>
                                    </NavItem>
                                    <NavGroup title="Demo Group">
                                        <NavItem is_active={self.cur_app_route == AppRoute::FormDemo}>
                                            <Link<AppRoute> to={AppRoute::FormDemo}>{"Form Demo"}</Link<AppRoute>>
                                        </NavItem>
                                        <NavItem is_active={self.cur_app_route == AppRoute::TableDemo}>
                                            <Link<AppRoute> to={AppRoute::TableDemo}>{"Table Demo"}</Link<AppRoute>>
                                        </NavItem>
                                    </NavGroup>
                                </Nav>
                            </PageSidebarBody>
                        </PageSidebar>
                    }}
                >
                    <Switch<AppRoute> render={switch} />
                </Page>
                {self.view_about_dialog(ctx)}
            </>
        }
    }
}

impl App {
    fn view_about_dialog(&self, ctx: &Context<Self>) -> Html {
        html! {
            <AboutModal
                is_open={self.about_dialog_is_open}
                onclose={ctx.link().callback(|_| Msg::ToggleAboutDialog)}
                trademark="Trademark"
                brand_image_src=""
                brand_image_alt=""
                product_name="Demo Product"
            >
            {"Demo about dialog info"}
            </AboutModal>
        }
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Dashboard => html! {<dashboard::Dashboard />},
        AppRoute::FormDemo => html! {<form_demo::FormDemo />},
        AppRoute::TableDemo => html! {<table_demo::TableDemo />},
    }
}
