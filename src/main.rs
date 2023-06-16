use axum::{routing::get, response::Html, Router};
use std::net::SocketAddr;

async fn hello() -> Html<&'static str>{ //manipulador
    Html(include_str!("hello.html"))
}


#[tokio::main]
async fn main(){

    let app = Router::new() //roteamento
        .route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); //definindo ip e porta

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()

}
