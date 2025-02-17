use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;


#[derive(Deserialize)]
struct FibInput{
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
    Html(format!("<h1>Fibonacci of {} is {}</h1>", input.val, result))
}

fn fib(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }
    return fib(x - 1) + fib(x - 2)
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;
    #[test]
    fn test_fib(){
        
        let inputs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let outputs = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

        for (i, o) in zip(inputs, outputs) {
            assert_eq!(fib(i), o);
        }
    }
}