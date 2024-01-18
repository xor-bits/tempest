# tempest

a simple html templating macro (prob unsafe to use atm)

```rust
async fn index(Path(v): Path<u8>) -> Html<String> {
    let view = view! {
        <html>
            <head>
                <title>"example"</title>
            </head>
            <body>
                // {r#"<script>alert("hello 1");</script>"#}
                // {unsanitized(r#"<script>alert("hello 2");</script>"#)}
                {app(v)}
            </body>
        </html>
    };

    Html(view.to_string())
}

fn app(v: u8) -> impl View {
    view! {
        <p>"Hello: " {v}</p>
    }
}
```
