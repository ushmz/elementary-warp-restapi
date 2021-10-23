use warp::Filter;

#[tokio::main]
async fn main() {
    let greeting = warp::path!("hello" / String)
        .map(|name: String| format!("Keep greeting to world forever, {}!", name));
    warp::serve(greeting)
        .run(([127, 0, 0, 1], 8000))
        .await;
}
