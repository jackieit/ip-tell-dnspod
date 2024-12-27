use crate::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <div class="container-row">
              <h1 class="page-title">{"Home"}</h1>
              <Link<Route> to={Route::UserLogin}>{"添加域名"}</Link<Route>>
            </div>
            <div class="container-row">
              <div class="table">
                <table class="table-striped" cellspacing="1">
                  <thead>
                    <tr>
                      <th>{"ID"}</th>
                      <th>{"主机名"}</th>
                      <th>{"域名"}</th>
                      <th>{"记录类型"}</th>
                      <th>{"记录值"}</th>
                      <th>{"操作"}</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr>
                      <td>{"1"}</td>
                      <td>{"www"}</td>
                      <th>{"example"}</th>
                      <td>{"A"}</td>
                      <td>{"192.168.1.1"}</td>
                      <td>{"修改"} {"|"} {"删除"}</td>
                    </tr>
                    <tr>
                      <td>{"2"}</td>
                      <td>{"@"}</td>
                      <th>{"example"}</th>
                      <td>{"AAAA"}</td>
                      <td>{"2408:8214:3619:26a0:3d9d:13c8:3c6c:8f8a"}</td>
                      <td>{"修改"} {"|"} {"删除"}</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
        </div>
    }
}