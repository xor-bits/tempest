use axum::{extract::Path, response::Html, routing::get};
use tempest::*;
use tokio::net::TcpListener;

use anyhow::Result;

//

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:60000";
    let listener = TcpListener::bind(addr).await?;

    let router = axum::Router::new()
        .route("/:id", get(index))
        .route("/more", get(more))
        .into_make_service();

    println!("listening on `http://{addr}/`");
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index(Path(v): Path<u8>) -> Html<String> {
    let view = view! {
        <html>
            <head>
                <title> "some nice title" </title>
                <script src="https://unpkg.com/htmx.org@1.9.10" integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC" crossorigin="anonymous"></script>
            </head>
            <body>
                // {r#"<script>alert("hello 1");</script>"#}
                // {unsanitized(r#"<script>alert("hello 2");</script>"#)}
                <h2>"path: " {v}</h2>
                {app()}
            </body>
        </html>
    };

    tracing::info!("sending `{}`", view.display());

    Html(view.to_string())
}

async fn more() -> Html<String> {
    Html(app_more().to_string())
}

fn app() -> impl View {
    view! {
        <table>
            <tr>
                <th> "a" </th>
                <th> "b" </th>
                <th> "c" </th>
            </tr>
            {app_more()}
        </table>
    }
}

fn app_more() -> impl View {
    view! {
        <tr>
            <td> "1" </td>
            <td> "2" </td>
            <td> "3" </td>
        </tr>
        <tr id="replaceMe">
            <td colspan="3">
                <button class="btn" hx-get="/more"
                                    hx-target="#replaceMe"
                                    hx-swap="outerHTML">
                    "load more"
                    <img class="htmx-indicator" src="/img/bars.svg" />
                </button>
            </td>
        </tr>
    }
}
