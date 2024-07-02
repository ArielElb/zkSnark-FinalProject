use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use prime_snarks::arkworks::backend::linear_equations::prove_linear_equations;
use prime_snarks::arkworks::backend::matrix_proof::prove_matrix;
use prime_snarks::arkworks::backend::matrix_proof::verify_proof;
use prime_snarks::sp1::miller_rabin::script::src::main::generate_proof;

fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route(
                "/prove_linear_equations",
                web::post().to(prove_linear_equations),
            )
            .route("/prime_sp1", web::post().to(generate_proof))
            .route("/matrix_prove/verify", web::post().to(verify_proof))
            .route("/matrix_prove/prove", web::post().to(prove_matrix)),
    );
}

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
