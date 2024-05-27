use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::get().to(subscribe))
            .route("/{name}", web::get().to(greet))
            .route("/", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe() -> impl Responder {
    let todo_panics = std::panic::catch_unwind(|| todo!("/subscribe route incomplete")).is_err();
    assert!(todo_panics);
    HttpResponse::Ok().finish()
}
