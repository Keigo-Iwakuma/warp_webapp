use warp::Filter;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let echo = warp::post()
        .and(warp::path("echo"))
        .and(warp::body::json())
        .map(|msg: Message| warp::reply::json(&msg));

    let routes = hello.or(echo);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
