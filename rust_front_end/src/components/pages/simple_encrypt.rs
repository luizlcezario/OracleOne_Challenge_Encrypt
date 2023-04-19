use stylist::{yew::styled_component, style};
use yew::prelude::*;
use crate::components::organisms::{navbar::Navbar, body_simple::BodySimple};

#[styled_component(SimpleEncrypt)]
pub fn simple_encrypt() -> Html {
	let stylecheat = style!(
		r#"
			width: 100%;
			margin: 0 auto;
			height: 50px;
			align-items: center;
			justify-content: center;
			display: flex;
			a {
				text-decoration: none;
				color: #d1981d;
			}
			a:hover {
				color: #d1981d;
				text-decoration: underline;
			}
		"#).unwrap();
	html! {
		<>
			<Navbar />
			<main class="stylecheat">
				<BodySimple />
			</main>
			<footer class={stylecheat}>
				<div>

					<p>{"Challenge Oracle One Desenvolvido por "} <a href="https://www.linkedin.com/in/luiz-lima-cezario/"> {"luiz Lima Cezario"}</a></p>
				</div>
			</footer>
		</>
	}
}