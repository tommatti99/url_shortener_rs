use rocket_cors::{Cors, AllowedOrigins, CorsOptions};
use rocket::http::Method;

pub fn cors_rules() -> Cors {
    // Usando AllowedOrigins::some com tipo explícito para exatas e regex
    let allowed_origins = AllowedOrigins::some::<&str, &str>(
        &["https://url-shortener-rs.up.railway.app", "http://localhost:9000"],  // Origens exatas
        &[]  // Expressões regulares vazias
    );

    let cors_options = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(vec![Method::Post, Method::Get].into_iter().map(From::from).collect())
        .allow_credentials(true)
        .allowed_headers(rocket_cors::AllowedHeaders::all());

    cors_options.to_cors().expect("ERROR AT CORS")
}