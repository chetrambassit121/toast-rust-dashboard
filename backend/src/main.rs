use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Rust and Docker backend is running!")
}

#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on port 8080");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
