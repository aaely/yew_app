use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .bind("192.168.4.95:8080")?
    .run()
    .await
}