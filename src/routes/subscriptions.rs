use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct SubscriptionData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(_form: web::Form<SubscriptionData>) -> HttpResponse {
    // let todo_panics = std::panic::catch_unwind(|| todo!("/subscribe route incomplete")).is_err();
    // assert!(todo_panics);
    HttpResponse::Ok().finish()
}
