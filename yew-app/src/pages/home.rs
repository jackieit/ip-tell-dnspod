use crate::Route;

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::data_defined::{RecordListItem,PagationRecords};

use crate::utils::{request,RequestOptions};

#[function_component(Home)]
pub fn home() -> Html {
    let loading = use_state(|| true);
    let data = use_state(|| Vec::<RecordListItem>::new());
    {
      let data_clone = data.clone();
      let loading_clone = loading.clone();
      use_effect(
          move || {
              //let client = Client::new();
              let request_options = RequestOptions {
                  uri: "/records".to_string(),
                  ..Default::default()
              };

              spawn_local(async move {
                  match request::<PagationRecords>(request_options).await {
                      Ok(resp) => {
                          data_clone.set(resp.data);
                      }
                      Err(_) => {
                          // Handle error
                      }
                  }
                  loading_clone.set(false);
              });

              || ()
          }
      );
  }
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
                    {for data.iter().map(|record| html!{
                      <tr>
                      <td>{record.id.to_string()}</td>
                      <td>{&record.host}</td>
                      <th>{&record.domain}</th>
                      <td>{&record.ip_type}</td>
                      <td>{&record.ip}</td>
                        <td>{"修改"} {"|"} {"删除"}</td>
                      </tr>
                    })}
 
                  </tbody>
                </table>
              </div>
            </div>
        </div>
    }
}