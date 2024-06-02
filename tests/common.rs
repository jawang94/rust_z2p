use std::net::TcpListener;
use zero2prod::run;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to spawn app server");
    let _ = tokio::spawn(server);
    format!("http:127.0.0.1:{}", port)
}
