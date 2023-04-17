use yew::prelude::*;
mod components;

use components::atoms::nav_logo::NavLogo;


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <NavLogo logo="./assets/Logo.png" />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}