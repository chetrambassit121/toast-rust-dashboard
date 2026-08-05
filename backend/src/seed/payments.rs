use reqwest::Client;
use serde_json::json;
use std::env;
use uuid::Uuid;

pub async fn seed_payments(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("SQUARE_ACCESS_TOKEN")?;
    let base_url = env::var("SQUARE_BASE_URL")
        .unwrap_or_else(|_| "https://connect.squareupsandbox.com".to_string());
    let location_id = env::var("SQUARE_LOCATION_ID")?;

    let dummy_payments: Vec<u64> = vec![1000, 2500, 500, 7599];

    for amount in dummy_payments {
        let body = json!({
            "source_id": "cnon:card-nonce-ok",
            "idempotency_key": Uuid::new_v4().to_string(),
            "amount_money": {
                "amount": amount,
                "currency": "USD"
            },
            "location_id": location_id
        });

        let response = client
            .post(format!("{}/v2/payments", base_url))
            .bearer_auth(&token)
            .header("Square-Version", "2026-07-16")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let data: serde_json::Value = response.json().await?;

        if status.is_success() {
            println!("  ✅ Payment created: ${:.2}", amount as f64 / 100.0);
        } else {
            println!("  ❌ Failed payment → {:?}", data);
        }
    }

    Ok(())
}