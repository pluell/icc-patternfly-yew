use yew::prelude::*;
use yew_router::prelude::*;

mod app;
mod components;

#[component]
fn AppRoot() -> Html {
    html! {
        <BrowserRouter>
            <app::App />
        </BrowserRouter>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Info);

    yew::Renderer::<AppRoot>::new().render();
}
