use std::fmt::Display;

use axum::{response::Html, routing::get};
use tempest::{view, View};
use tokio::net::TcpListener;

use anyhow::Result;

//

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:60000";
    let listener = TcpListener::bind(addr).await?;

    let router = axum::Router::new()
        .route("/", get(index))
        .into_make_service();

    println!("listening on `http://{addr}/`");
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index() -> Html<String> {
    let view = view! {
        <html>
            <head>
                <title>Hello</title>
            </head>
            <body>
                <div>
                    <a href="file:///yeet">Hello</a>
                    <button attrs/>
                </div>
            </body>
        </html>
    };

    Html(view.to_string())
}

// fn app() -> impl ViewA {
//     let x = 4;
//     WrapView(move |f: &mut core::fmt::Formatter| -> core::fmt::Result {
//         f.write_str("<a></a>")?;
//         core::fmt::Display::fmt(&gen(), f)?;
//         core::fmt::Display::fmt(&x, f)?;
//         Ok(())
//     })
// }

// fn gen() -> impl ViewA {
//     WrapView(|f: &mut core::fmt::Formatter| -> core::fmt::Result {
//         f.write_str("<a></a>")?;
//         Ok(())
//     })
// }

// struct WrapView<F>(F);

// pub trait ViewA: core::fmt::Display {}

// impl<F: Fn(&mut core::fmt::Formatter) -> core::fmt::Result> ViewA for WrapView<F> {}

// impl<F: Fn(&mut core::fmt::Formatter) -> core::fmt::Result> core::fmt::Display for WrapView<F> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         (self.0)(f)
//     }
// }

// pub struct View {
//     _0: &'static str,
//     _1: &
// }
