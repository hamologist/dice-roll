use axum::{
    http::{StatusCode},
    extract::rejection::JsonRejection,
    routing::post,
    Json,
    Router
};
use dice_roll::{RollRequest};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(roll));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
                }))
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
