use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::Link;

use pages::user_login::UserLogin;

pub mod pages;
pub mod utils;
pub mod error;
/// defined router
#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/user/login")]
    UserLogin,
    #[at("/user/list")]
    UserList,
    #[at("/user/reset_password")]
    UserResetPassword,
    #[at("/app")]
    App,
    #[at("/domain")]
    Domain,
    #[at("/404")]
    NotFound
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        </>
    }
}
/// create a empty function component
#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <h1>{ "404 Not Found" }</h1>
    }
}
/// create a router switch
fn switch(route: Route) -> Html {
    match route {
        Route::UserLogin => html! {
            // Login page without layout
           <UserLogin />
        },
        Route::Home => html! {
            <Layout>
                <div>
                    <h1>{ "Home" }</h1>
                </div>
            </Layout>
        },
        _ => html! { <NotFound /> },
    }
}
#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}
// Create a Layout component
#[function_component(Layout)]
fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="app-container">
            <nav class="navbar">
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::UserList}>{ "Users" }</Link<Route>>
                <Link<Route> to={Route::App}>{ "App" }</Link<Route>>
                <Link<Route> to={Route::Domain}>{ "Domain" }</Link<Route>>
            </nav>
            
            <main class="main-content">
                { props.children.clone() }
            </main>
            
            <footer class="footer">
                { "© 2024 Your App Name" }
            </footer>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}