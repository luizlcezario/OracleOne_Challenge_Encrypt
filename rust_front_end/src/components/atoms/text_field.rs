use stylist::{yew::styled_component, style};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props {
	pub name: String,
	pub handle_change: Callback<String>,
}

#[styled_component(TextField)]
pub fn text_field(props:&Props) -> Html {
	let changes = props.handle_change.clone();
	let on_change = Callback::from(move |e: Event| {
		let target = e.target().unwrap();
		let input = target.unchecked_into::<HtmlInputElement>().value();
		changes.emit(input);
	});
	let stylelist = style!(
		r#"
			min-height: 400px;
			width: 100%;
			height: 100%;
			word-break: break-word;
			border: 1px solid #ccc;
			border-radius: 4px;
			padding: 12px 20px;
			box-sizing: border-box;
			margin-bottom: 20px;
			background-color: #dddddd;
			color: #d1981d;
			font-size: 1.4rem;
			resize: none;
			scrollbar: none;
			border: none;
		"#).unwrap();
	html! {
		<textarea placeholder="Let`s Rust your string!" class={stylelist} name={props.name.clone()} onchange={on_change}> </textarea>
	}
}