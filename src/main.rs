mod views;
use actix_web::{HttpRequest, Responder, HttpServer, App, web};
use views::views_factory;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(views::views_factory)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
