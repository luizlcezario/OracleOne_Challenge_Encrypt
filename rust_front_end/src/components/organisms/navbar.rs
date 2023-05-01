use stylist::{yew::styled_component, style};
use yew::prelude::*;
use crate::components::atoms::nav_logo::NavLogo;


#[styled_component(Navbar)]
pub fn navbar() -> Html {
	let styles = style!(
		r#"
				display: flex;
				align-items: center;
				height: 80px;
				margin-left: 10px;
		"#).unwrap();
	html! {
		<header class={styles}>
			<a href="https://luizlcezario.github.io/OracleOne_Challenge_Encrypt/">
				<NavLogo logo="./assets/Logo.png" />
			</a>
		</header>
	}
}