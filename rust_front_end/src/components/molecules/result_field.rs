use stylist::{yew::styled_component, style};
use yew::prelude::*;
use crate::components::organisms::body_simple::Status;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub response: String,
	pub status: Status,
}


#[styled_component(ResultField)]
pub fn result_field(props: &Props) -> Html {
	let stylecheat = style!(
		r#"
			display: flex;
			flex-direction: column;
			margin: 30px auto;
			h1 {
				color: #d1981d;
			}
			p {
				color: #505050;
			}
			img {
				display: none;
			}
			@media (min-width: 530px) {
				img {
					width: 90%;
					height: 90%;
					max-width: 400px;
					margin: 0 auto;
					object-fit: cover;
					display: block;
				}
			}
		"#).unwrap();
	html! {
		<>
		if props.response != "" && props.status == Status::OK {
			<div class={stylecheat}>
				<h1>{props.response.clone()}</h1>
			</div>
		} else {
			<div class={stylecheat}>
				<img src="/assets/background.png" />
				<h1>{"Nenhuma mensagem encontrada"}</h1>
				<p> {"Digite um texto que você deseja criptografar ou descriptografar."} </p>
			</div>
		}
		</>
	}
}