use actix_web::{
    get, post, App, HttpResponse, HttpServer, Responder,
};
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::env;

mod seed;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .body("Rust and Docker backend is running!")
}

#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

async fn square_get(path: &str) -> HttpResponse {
    let access_token = match env::var("SQUARE_ACCESS_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                    "error": "SQUARE_ACCESS_TOKEN is missing"
                }),
            );
        }
    };

    let base_url = env::var("SQUARE_BASE_URL")
        .unwrap_or_else(|_| {
            "https://connect.squareupsandbox.com".to_string()
        });

    let client = Client::new();

    let response = client
        .get(format!("{base_url}{path}"))
        .bearer_auth(access_token)
        .header("Square-Version", "2026-07-16")
        .header("Content-Type", "application/json")
        .send()
        .await;

    match response {
    Ok(square_response) => {
        let status = square_response.status();

        let body = match square_response.text().await {
            Ok(body) => body,
            Err(error) => {
                return HttpResponse::InternalServerError().json(
                    serde_json::json!({
                        "error": "Could not read Square response",
                        "details": error.to_string()
                    }),
                );
            }
        };

        println!("Square status: {}", status);
        println!("Square response: {}", body);

        if status.is_success() {
            match serde_json::from_str::<Value>(&body) {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(_) => HttpResponse::Ok().body(body),
            }
        } else {
            HttpResponse::build(
                actix_web::http::StatusCode::from_u16(status.as_u16())
                    .unwrap_or(actix_web::http::StatusCode::BAD_GATEWAY),
            )
            .json(serde_json::json!({
                "error": "Square API request failed",
                "square_status": status.as_u16(),
                "square_response": body
            }))
        }
    }

    Err(error) => {
        HttpResponse::InternalServerError().json(
            serde_json::json!({
                "error": "Could not connect to Square",
                "details": error.to_string()
            }),
        )
    }
}

#[get("/api/square/locations")]
async fn square_locations() -> impl Responder {
    square_get("/v2/locations").await
}

#[get("/api/square/customers")]
async fn square_customers() -> impl Responder {
    square_get("/v2/customers").await
}

#[get("/api/square/catalog")]
async fn square_catalog() -> impl Responder {
    square_get("/v2/catalog/list?types=ITEM,CATEGORY").await
}

#[get("/api/square/payments")]
async fn square_payments() -> impl Responder {
    square_get("/v2/payments").await
}

#[post("/api/seed")]
async fn run_seeder() -> impl Responder {
    let client = Client::new();

    println!("🌱 Starting Square Sandbox Seeder...");

    let mut errors: Vec<String> = vec![];

    if let Err(error) =
        seed::customers::seed_customers(&client).await
    {
        errors.push(format!("Customers error: {error}"));
    }

    if let Err(error) =
        seed::catalog::seed_catalog(&client).await
    {
        errors.push(format!("Catalog error: {error}"));
    }

    if let Err(error) =
        seed::payments::seed_payments(&client).await
    {
        errors.push(format!("Payments error: {error}"));
    }

    if errors.is_empty() {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "All dummy data seeded successfully!"
        }))
    } else {
        HttpResponse::InternalServerError().json(
            serde_json::json!({
                "status": "partial_failure",
                "errors": errors
            }),
        )
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(health)
            .service(square_locations)
            .service(square_customers)
            .service(square_catalog)
            .service(square_payments)
            .service(run_seeder)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}