use stylist::{yew::styled_component, style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub logo: String,
}

#[styled_component(NavLogo)]
pub fn nav_logo(props: &Props) -> Html {
	let stylecheat = style!(
		r#"
			height: 80px;
			margin-right: 10px;
			margin-bottom: 10px;
		"#).unwrap();
	let Props { logo } = props;
	html! {
		<img class={stylecheat} src={logo.clone()} alt="logo" />
	}
}