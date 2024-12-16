use yew::prelude::*;

#[function_component(UserLogin)]
pub fn user_login() -> Html {
    html! {
        <div class="login-container">
            <div class="login-container-panel">
                <div class="login-container-panel-title">{"管理登录"}</div>
                <div class="login-container-panel-form">
                    <div class="login-container-panel-form-item">
                        <input class="login-container-panel-form-item-input" type="text" placeholder="请输入用户名" />
                    </div>
                    <div class="login-container-panel-form-item">
                        <input class="login-container-panel-form-item-input" type="password" placeholder="请输入密码" />
                    </div>
                    <div class="login-container-panel-form-item login-container-panel-form-button-area">
                        <button class="login-container-panel-form-item-button">{"登录"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
