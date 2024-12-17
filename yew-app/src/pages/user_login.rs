use yew::prelude::*;
use web_sys::{EventTarget, HtmlInputElement};
use web_sys::wasm_bindgen::JsCast;

#[derive(Default,Clone,PartialEq)]
pub struct LoginForm {
    username: String,
    password: String,
}
#[function_component(UserLogin)]
pub fn user_login() -> Html {
  let input_value_handle = use_state(LoginForm::default);
  let input_value = (*input_value_handle).clone();
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
                "username" => LoginForm { username: value, ..current_form },
                "password" => LoginForm { password: value, ..current_form },
                _ => current_form,
            });
            }
           
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
                        <button class="login-container-panel-form-item-button">{"登录"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
