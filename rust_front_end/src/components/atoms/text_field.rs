
use stylist::{yew::styled_component, style};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::organisms::body_simple::Status;

#[derive(Properties,PartialEq)]
pub struct Props {
	pub name: String,
	pub handle_change: Callback<String>,
	pub status: Status,
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
	
	if props.status == Status::INFO {
		let stylelist2 = style!(r#"
		::placeholder { /* Chrome, Firefox, Opera, Safari 10.1+ */
			color:  #d81d35;
			opacity: 1; /* Firefox */
		  }
		"#).unwrap();
		html! {
			<textarea placeholder="Let`s Rust your string!" class={classes!(stylelist,stylelist2)}name={props.name.clone()} onchange={on_change}> </textarea>
		}
	} else {
			html! {
			<textarea placeholder="Let`s Rust your string!" class={stylelist} name={props.name.clone()} onchange={on_change}> </textarea>
		}
	}
}