use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::env;

// ─────────────────────────────────────────────
// 🏠 Basic Routes
// ─────────────────────────────────────────────

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

// ─────────────────────────────────────────────
// 📍 Square Locations Route
// ─────────────────────────────────────────────

#[get("/api/square/locations")]
async fn square_locations() -> impl Responder {
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
        .unwrap_or_else(|_| "https://connect.squareupsandbox.com".to_string());

    let client = Client::new();

    let response = client
        .get(format!("{base_url}/v2/locations"))
        .bearer_auth(access_token)
        .header("Square-Version", "2026-07-15")
        .header("Content-Type", "application/json")
        .send()
        .await;

    match response {
        Ok(square_response) => {
            let status = square_response.status();

            match square_response.json::<Value>().await {
                Ok(data) => {
                    if status.is_success() {
                        HttpResponse::Ok().json(data)
                    } else {
                        HttpResponse::build(
                            actix_web::http::StatusCode::from_u16(
                                status.as_u16()
                            )
                            .unwrap_or(
                                actix_web::http::StatusCode::BAD_GATEWAY
                            ),
                        )
                        .json(data)
                    }
                }

                Err(error) => {
                    HttpResponse::InternalServerError().json(
                        serde_json::json!({
                            "error": "Could not read Square response",
                            "details": error.to_string()
                        }),
                    )
                }
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
}

// ─────────────────────────────────────────────
// 👥 Square Customers Route
// ─────────────────────────────────────────────

#[get("/api/square/customers")]
async fn square_customers() -> impl Responder {
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
        .unwrap_or_else(|_| "https://connect.squareupsandbox.com".to_string());

    let client = Client::new();

    let response = client
        .get(format!("{base_url}/v2/customers"))
        .bearer_auth(access_token)
        .header("Square-Version", "2026-07-15")
        .header("Content-Type", "application/json")
        .send()
        .await;

    match response {
        Ok(square_response) => {
            let status = square_response.status();

            match square_response.json::<Value>().await {
                Ok(data) => {
                    if status.is_success() {
                        HttpResponse::Ok().json(data)
                    } else {
                        HttpResponse::build(
                            actix_web::http::StatusCode::from_u16(
                                status.as_u16()
                            )
                            .unwrap_or(
                                actix_web::http::StatusCode::BAD_GATEWAY
                            ),
                        )
                        .json(data)
                    }
                }

                Err(error) => {
                    HttpResponse::InternalServerError().json(
                        serde_json::json!({
                            "error": "Could not read Square response",
                            "details": error.to_string()
                        }),
                    )
                }
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
}

// ─────────────────────────────────────────────
// 🚀 Main Entry Point
// ─────────────────────────────────────────────

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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}