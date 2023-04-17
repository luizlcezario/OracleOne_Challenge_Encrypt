use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub logo: String,
}

#[function_component(NavLogo)]
pub fn nav_logo(props: &Props) -> Html {
	let Props { logo } = props;
	html! {
		<img src={logo.clone()} alt="logo" />
	}
}