use stylist::{yew::styled_component};
use stylist::css;
use stylist::yew::Global;
use yew::prelude::*;
mod components;
use components::pages::simple_encrypt::SimpleEncrypt;


#[styled_component(App)]
fn app() -> Html {
    let stylecheat = css!(
        r#"
            width: 100%;
            height: 100%;
            margin: 0;
            padding: 0;
            box-sizing: border-box;
            font-family: 'Inter', sans-serif;
            background-color: #505050;
            color: #dddddd;
        "#);
    html! {
        <>
            <Global css={stylecheat} />
            <SimpleEncrypt />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}