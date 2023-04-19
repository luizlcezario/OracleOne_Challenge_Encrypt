use stylist::{yew::styled_component, style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub label: String,
	pub inverted: bool,
}

#[styled_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
	let stylecheat = style!(
		r#"
			max-width: 500px;
			min-width: 300px;
			width: 45%;
			height: 50px;
			border: none;
			border-radius: 10px;
			color: #fff;
			background-color: #c4901f;
			font-size: 1.2rem;
			font-weight: 600;
			transition: 400ms;
			cursor: pointer;
			margin: 10px ;
			:hover {
				border: 1px solid #c4901f;
				background-color: #dddddd;
				color: #c4901f;
			}
		"#).unwrap();
		let stylecheat2 = style!(
			r#"
				max-width: 500px;
				min-width: 300px;
				width: 45%;
				height: 50px;
				border: none;
				border-radius: 10px;
				font-size: 1.2rem;
				font-weight: 600;
				transition: 400ms;
				cursor: pointer;
				margin: 10px ;
				background-color: #dddddd;
				border: 1px solid #c4901f;
				color: #c4901f;
				:hover {
					color: #fff;
					background-color: #c4901f;
				}
			"#).unwrap();
	html! {
		if props.inverted {
			<input class={stylecheat2} type="submit" placeholder={props.label.clone()} name="button" value={props.label.clone()}/>
		} else {
			<input class={stylecheat} type="submit" placeholder={props.label.clone()} name="button" value={props.label.clone()}/>
		}
	}
}