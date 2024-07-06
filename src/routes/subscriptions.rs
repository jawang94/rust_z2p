use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    // let todo_panics = std::panic::catch_unwind(|| todo!("/subscribe route incomplete")).is_err();
    // assert!(todo_panics);
    HttpResponse::Ok().finish()
}
