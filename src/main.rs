use axum::extract::FromRef;
use leptos::LeptosOptions;
use std::sync::Arc;

#[cfg(feature = "ssr")]
#[derive(Clone, FromRef)]
struct State {
    leptos_options: Arc<LeptosOptions>,
}
#[cfg(feature = "ssr")]
impl leptos_axum::LeptosOptionProvider for State {
    fn options(&self) -> LeptosOptions {
        (*self.leptos_options).clone()
    }
}

#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use start_axum::app::*;
    use start_axum::fileserv::file_and_error_handler;
    use std::sync::Arc;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let state = State {
        leptos_options: Arc::new(leptos_options),
    };

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        // leptos_routes seems to take us from a Router<S> to a Router<()>,
        // i.e. force S = ()?
        .leptos_routes(state.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
