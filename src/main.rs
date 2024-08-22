use actix_files as fs;
use actix_web::{get, App, HttpServer, Responder};

#[get("/api/hello")]
async fn hello() -> impl Responder {
    "Hello from Actix Web!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(fs::Files::new("/", "./public").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
