use axum::{extract::Query, response::Html, routing::get, Router};
use memoize::memoize;
use serde::Deserialize;

#[derive(Deserialize)]
struct FibInput {
    val: i32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/fib", get(h_fib));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn h_fib(input: Query<FibInput>) -> Html<String> {
    let result = fib(input.val);
    match result {
        Some(result) => Html(format!("<h1>Fibonacci of {} is {}</h1>", input.val, result)),
        None => Html(format!(
            "<h1>Fibonacci of {} is out of range</h1>",
            input.val
        )),
    }
}

#[memoize]
fn fib(x: i32) -> Option<i32> {
    match x {
        0 => Some(0),
        1 => Some(1),
        _ => fib(x - 1)?.checked_add(fib(x - 2)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fib() {
        let cases = [
            (0, Some(0)),
            (1, Some(1)),
            (6, Some(8)),
            (9, Some(34)),
            // Testing the overflow threshold for i32
            (46, Some(1836311903)),
            (47, None), // fib(47) overflows i32
        ];

        for (i, o) in cases {
            assert_eq!(fib(i), o, "Failed at fib({})", i);
        }
    }
}
