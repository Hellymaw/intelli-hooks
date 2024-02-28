use axum::{extract::Json, routing::post, Router};
use serde_json;

pub mod gitea_webhooks;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(post_handler));

    let listerner = tokio::net::TcpListener::bind("192.168.0.26:6969")
        .await
        .unwrap();

    axum::serve(listerner, app).await.unwrap();
}

async fn post_handler(Json(payload): Json<serde_json::Value>) {
    if let Ok(webhook) = serde_json::from_value::<gitea_webhooks::Webhook>(payload) {
        match webhook.action.as_str() {
            "review_requested" => review_requested(webhook),
            "reviewed" => reviewed(webhook),
            "opened" => opened(webhook),
            action => println!("Unhandled action \"{}\"", action),
        }
    } else {
        println!("Issue deserializing JSON!");
    }
}

fn review_requested(payload: gitea_webhooks::Webhook) {
    let requester = payload.sender.email;
    let requested = payload.requested_reviewer.unwrap().email;

    println!("{} requested a review from {}", requester, requested);
}

fn reviewed(payload: gitea_webhooks::Webhook) {
    let reviewer = payload.sender.email;
    let review = payload.review.unwrap();

    println!("{} {:?} the pull-request", reviewer, review.r#type);
}

fn opened(payload: gitea_webhooks::Webhook) {
    let opener = payload.sender.email;
    let title = payload.pull_request.title;
    let body = payload.pull_request.body;
    let number = payload.pull_request.id;

    println!("{} opened PR#{} \"{}\"\n{}", opener, number, title, body);
}
