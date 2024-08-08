#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use redis::AsyncCommands;
use redis::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct GoroutinePayload {
    goroutine_id: String,
    timestamp: String,
}

#[derive(Clone)]
struct AppState {
    redis_client: Arc<Mutex<redis::aio::Connection>>,
}

#[post("/trigger")]
async fn trigger_goroutines() -> &'static str {
    println!("Triggering Goroutines...");
    "Goroutines triggered"
}

#[post("/receive", format = "application/json", data = "<payload>")]
async fn receive_payload(payload: Json<GoroutinePayload>, state: &rocket::State<AppState>) -> &'static str {
    println!("Received payload: {:?}", payload);

    let mut con = state.redis_client.lock().await;

    // Convert the payload timestamp to a String
    let timestamp = payload.timestamp.clone();

    // Use the explicit type for rpush
    match con.rpush::<&str, String, ()>(&payload.goroutine_id, timestamp).await {
        Ok(_) => {
            println!("Successfully stored payload in Redis");
            "Payload received and stored in Redis"
        }
        Err(err) => {
            eprintln!("Failed to store payload in Redis: {}", err);
            "Failed to store payload in Redis"
        }
    }
}

#[rocket::main]
async fn main() {
    match Client::open("redis://127.0.0.1:6380/") {
        Ok(client) => match client.get_async_connection().await {
            Ok(connection) => {
                let state = AppState {
                    redis_client: Arc::new(Mutex::new(connection)),
                };
                rocket::build()
                    .manage(state)
                    .mount("/", routes![trigger_goroutines, receive_payload])
                    .launch()
                    .await
                    .unwrap();
            }
            Err(err) => {
                eprintln!("Failed to establish Redis connection: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Failed to create Redis client: {}", err);
        }
    }
}

