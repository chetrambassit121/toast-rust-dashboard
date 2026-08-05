use reqwest::Client;
use serde_json::json;
use std::env;
use uuid::Uuid;

pub async fn seed_customers(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("SQUARE_ACCESS_TOKEN")?;
    let base_url = env::var("SQUARE_BASE_URL")
        .unwrap_or_else(|_| "https://connect.squareupsandbox.com".to_string());

    let dummy_customers = vec![
        ("John", "Doe", "johndoe@example.com"),
        ("Jane", "Smith", "janesmith@example.com"),
        ("Bob", "Johnson", "bobjohnson@example.com"),
    ];

    for (first, last, email) in dummy_customers {
        let body = json!({
            "given_name": first,
            "family_name": last,
            "email_address": email,
            "idempotency_key": Uuid::new_v4().to_string()
        });

        let response = client
            .post(format!("{}/v2/customers", base_url))
            .bearer_auth(&token)
            .header("Square-Version", "2026-07-16")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let data: serde_json::Value = response.json().await?;

        if status.is_success() {
            println!("  ✅ Customer created: {} {}", first, last);
        } else {
            println!("  ❌ Failed: {} {} → {:?}", first, last, data);
        }
    }

    Ok(())
}