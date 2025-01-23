
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::net::TcpListener;

    use axum::Router;
    use axum_server::tls_rustls::RustlsConfig;
    use clap::Parser;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use webstack::{app::*, server::utils::clap::Opt};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);


    let opts = Opt::parse();

    let listener = TcpListener::bind(&addr)
        .unwrap();
    
    let config = RustlsConfig::from_pem_file(
        opts.cert,
        opts.key
    )
        .await
        .unwrap();

    log!("listening on https://{}", &addr);

    axum_server::from_tcp_rustls(listener, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
