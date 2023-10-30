use actix_web::{delete, get, patch, post, HttpResponse, Responder};

#[delete("/accounts/{account_id}")]
async fn delete_account() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/accounts/{account_id}")]
async fn get_account() -> impl Responder {
    HttpResponse::Ok()
}

#[patch("/accounts/{account_id}")]
async fn update_account() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/accounts/{account_id}")]
async fn create_account() -> impl Responder {
    HttpResponse::Created()
}
