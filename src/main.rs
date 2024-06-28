use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use prime_snarks::arkworks::backend::linear_equations::prove_linear_equations;
use prime_snarks::arkworks::backend::prime_snark::prime_snark_compute;

fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/prime_snark", web::post().to(prime_snark_compute))
            .route(
                "/prove_linear_equations",
                web::post().to(prove_linear_equations),
            ),
    );
}

// for final deployment to production
fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(Files::new("/", "./build").index_file("index.html"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .configure(configure_services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
