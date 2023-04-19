use aho_corasick::AhoCorasick;
use stylist::{yew::styled_component, style};
use yew::prelude::*;

use crate::components::molecules::{custom_form::{CustomForm, State, Crypt}, result_field::ResultField};

#[derive(PartialEq, Default, Clone, Copy)]
pub enum Status{
	#[default]
	OK,
	ERROR,
	WARN,
	INFO,
}
#[derive(Default)]
struct Response{
	text: String,
	type_crypt: Crypt,
	response: String,
	status: Status,
}

impl Response {
	fn res_generate(& mut self) {
		let patterns = ["a", "e","i", "o", "u"];
		let replaces = ["ai", "enter", "imes", "ober", "ufat"];
		if self.type_crypt == Crypt::Encrypt {
			self.response = Response::crypt(&self.text, &patterns, &replaces);
		} else if self.type_crypt == Crypt::Decrypt {
			self.response = Response::crypt(&self.text, &replaces, &patterns);
		}
		else {
			self.response = String::from("");
		}
	}
	fn crypt(text: &String, patterns: &[&str], replaces: &[&str]) -> String {
		let ac = AhoCorasick::new(patterns).unwrap();
		let result = ac.replace_all(&text, replaces);
		return result;
	}

	fn validation(&mut self) {
		if self.text == "" {
			self.status == Status::INFO;
		}
		else
		{
			let regex = "/[A-Z\u{00C0}-\u{024F}]/g";
			let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
			self.status = Status::OK;
		}
	}
}

#[styled_component(BodySimple)]
pub fn body_simple() -> Html {
	let res_state = use_state(|| Response::default());
	let cloned_res = res_state.clone();
	let onsubmit = Callback::from( move |state: State| {
		let mut res = Response {
			text: state.text,
			type_crypt: state.type_crypt,
			response : String::from(""),
			status: Status::OK,
		};
		res.validation();
		res.res_generate();
		cloned_res.set(res);
	});
	let stylecheat = style!(
		r#"
			margin-top: 30px;
			display: grid;
			grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
			grid-gap: 50px 20px;
			> div {
				display: flex;
				flex-direction: column;
				min-height: 150px;
				background: #dddddd;
				width: 100%
				padding: 10px;
				align-items: center;
				align-content: space-between;
				text-align: center;
				height: 100%;
				box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
				border-radius: 24px;
			}
		"#).unwrap();
	html! {
		<div class={stylecheat}>
			<div>
				<CustomForm onsubmit={onsubmit} />
			</div>
			<div>
				<ResultField response={res_state.response.clone()} status={res_state.status} />
			</div>
		</div>
	}
}

