use std::ops::Deref;
use stylist::style;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*, functional::use_state};
use crate::components::atoms::text_field::TextField;
use crate::components::atoms::custom_button::CustomButton;
use crate::components::organisms::body_simple::Status;

#[derive(Clone, Default, PartialEq, Debug)]
pub enum Crypt {
	#[default]
	Encrypt,
	Decrypt,
}

#[derive(Clone, Default)]
pub struct State {
	pub text: String,
	pub type_crypt: Crypt,
}

#[derive(Properties, PartialEq)]
pub struct Props {
	pub onsubmit: Callback<State>,
	pub status: Status,
}

#[styled_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
	let state = use_state(|| State::default());
	let cloned_state = state.clone();
	let text_onchange = Callback::from(
			move |text:String| {
				cloned_state.set(
					State {
						text,
						..cloned_state.deref().clone()
					}
				)
		});
	let onsubmit_handle = props.onsubmit.clone();
	let onsubmit = Callback::from(
			move |event: SubmitEvent| {
				event.prevent_default();
				let target = event.submitter().unwrap().unchecked_into::<HtmlInputElement>().value();
				if target.to_lowercase() == "encrypt" {
					onsubmit_handle.emit(
						State {
							type_crypt: Crypt::Encrypt,
							..state.deref().clone()
						}
					)
				} else if target.to_lowercase() == "decrypt" {
					onsubmit_handle.emit(
						State {
							type_crypt: Crypt::Decrypt,
							..state.deref().clone()
						}
					)
				}
				else {
					onsubmit_handle.emit(state.deref().clone());
				}
		});
		let stylecheat_error = style!(
			r#"
				span {
					color: #d81d35;
					font-size: 15px;
				}
				span img {
					filter: hue-rotate(134deg) saturate(3) brightness(1.9) contrast(3);
				}
			"#).unwrap();
		let stylecheat_ok = style!(
				r#"
				span {
					color: #343A40;
					background: transparent;
				}
				"#).unwrap();
		let stylecheat = style!(
			r#"
				margin: 40px;
				span img {
					vertical-align: middle;
					width: 16px;
					margin-right: 5px;
				}
			"#).unwrap();
			let status = props.status.clone();
	html! {
		<form  onsubmit={onsubmit}>
			<div class={stylecheat}>
				<TextField name="text_crypt" handle_change={text_onchange} status={status} />
				<div  class={if status == Status::ERROR {stylecheat_error} else {stylecheat_ok}}>
					<span>
						<img src="assets/icon.png" alt="info" />
						{"Apenas letras minúsculas e sem acento."}
					</span>
				</div>
				<CustomButton label="Encrypt" inverted={false}/>
				<CustomButton label="Decrypt" inverted={true}/>
			</div>
		</form>
	}
}