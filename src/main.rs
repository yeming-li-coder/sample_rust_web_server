use actix_web::{App, HttpServer};
mod processes;
mod state;
mod to_do;
mod views;
mod jwt;
mod json_serialization;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(views::views_factory))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
