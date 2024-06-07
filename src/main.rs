mod backend;
mod constraints;
mod miller_rabin;

use actix_cors::Cors;
use actix_files::Files;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(backend::prime_snark::compute)
            .service(Files::new("/", "./build").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
