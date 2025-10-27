use axum::{Json, Router, extract::rejection::JsonRejection, http::StatusCode, routing::post};
use clap::Parser;
use dice_roll::RollRequest;
use serde_json::{Value, json};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    #[arg(long, default_value_t = 3000)]
    port: i32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let app = Router::new().route("/", post(roll));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port))
        .await
        .unwrap();
    let local_addr = match listener.local_addr() {
        Ok(val) => val,
        Err(_) => {
            println!("Failed find local address server is running on.");
            return;
        }
    };
    println!(
        "Server running on {}:{}",
        local_addr.ip(),
        local_addr.port()
    );
    axum::serve(listener, app).await.unwrap();
}

pub async fn roll(payload: Result<Json<RollRequest>, JsonRejection>) -> (StatusCode, Json<Value>) {
    let roll_request = match payload {
        Ok(roll_request) => roll_request,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": "INVALID_JSON",
                    "message": e.to_string()
                })),
            );
        }
    };
    match roll_request.roll_dice() {
        Ok(roll_response) => {
            return (StatusCode::OK, Json(roll_response.to_json()));
        }
        Err(e) => {
            return (StatusCode::BAD_REQUEST, Json(e.to_json()));
        }
    }
}
