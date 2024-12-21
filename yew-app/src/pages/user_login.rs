use gloo_console::log;
use gloo_net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Default, Clone, PartialEq, Serialize, Debug)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[function_component(UserLogin)]
pub fn user_login() -> Html {
    let input_value_handle = use_state(LoginForm::default);
    let input_value = (*input_value_handle).clone();
    let loading = use_state(|| false);
    let _error_message = use_state(String::default);
    let on_input_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            // get input name and value

            if let Some(input) = input {
                let name = input.name();
                let value = input.value();
                let current_form = (*input_value_handle).clone();

                input_value_handle.set(match name.as_str() {
                    "username" => LoginForm {
                        username: value,
                        ..current_form
                    },
                    "password" => LoginForm {
                        password: value,
                        ..current_form
                    },
                    _ => current_form,
                });
            }
        })
    };
    let on_submit = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            log!("submit");
            let current_form = (*input_value_handle).clone();
            let loading = loading.clone();
            loading.set(true);
            spawn_local(async move {
                let response = Request::post("http://localhost:3310/v1/user/signin")
                    .json(&current_form)
                    .expect("Failed to serialize request data")
                    //.headers(vec![("Content-Type", "application/json")])
                    .send()
                    .await
                    .unwrap();
                let response_body = response.text().await.unwrap();
                log!("response_body: {:?}", response_body);
                loading.set(false);
            });
        })
    };
    html! {
        <div class="login-container">
            <div class="login-container-panel">
                <div class="login-container-panel-title">{"管理登录"}</div>
                <div class="login-container-panel-form">
                    <div class="login-container-panel-form-item">
                        <input class="login-container-panel-form-item-input"
                        type="text" name="username" placeholder="请输入用户名"
                        value={input_value.username.clone()}
                        onchange={on_input_change.clone()} />
                    </div>
                    <div class="login-container-panel-form-item">
                        <input class="login-container-panel-form-item-input"
                        type="password" name="password" placeholder="请输入密码"
                        value={input_value.password.clone()}
                        onchange={on_input_change.clone()} />
                    </div>
                    <div class="login-container-panel-form-item login-container-panel-form-button-area">
                        <button class="login-container-panel-form-item-button" onclick={on_submit}>{"登录"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
