use actix_web::{web, App, HttpServer};

mod cache;
mod controllers;
mod models;
mod repository;
mod validation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/v1alpha1")
                .service(controllers::create_account)
                .service(controllers::delete_account)
                .service(controllers::get_account)
                .service(controllers::update_account),
        )
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
