use axum::{Json, Router, extract::rejection::JsonRejection, http::StatusCode, routing::post};
use clap::{self, ArgAction};
use dice_roll::RollRequest;
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let matches = clap::Command::new("dice-roll-api")
        .about("Dice rolls as a service")
        .arg(
            clap::Arg::new("host")
                .long("host")
                .default_value("0.0.0.0")
                .action(ArgAction::Set)
                .help("Host to run the webserver on."),
        )
        .arg(
            clap::Arg::new("port")
                .long("port")
                .default_value("3000")
                .action(ArgAction::Set)
                .help("Port to run the webserver on."),
        )
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<String>("port").unwrap();

    let app = Router::new().route("/", post(roll));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
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
