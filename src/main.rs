use axum::{extract::Json, routing::post, Router};
use reqwest;
use serde_json;

pub mod gitea_webhooks;

const BIND_ADDRESS: &str = "192.168.0.26:6969";
const SLACK_WEBHOOK_ADDRESS: &str = "http://localhost:8000/";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(post_handler));
    let listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn send_slack_webhook(body: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(SLACK_WEBHOOK_ADDRESS)
        .body(body.to_string())
        .send()
        .await?;

    println!("body = {:?}", res);

    res.error_for_status()
}

async fn post_handler(Json(payload): Json<serde_json::Value>) {
    if let Ok(webhook) = serde_json::from_value::<gitea_webhooks::Webhook>(payload) {
        match webhook.action.as_str() {
            "review_requested" => review_requested(webhook).await,
            "reviewed" => reviewed(webhook).await,
            "opened" => opened(webhook).await,
            action => println!("Unhandled action \"{}\"", action),
        }
    } else {
        println!("Issue deserializing JSON!");
    }
}

async fn review_requested(payload: gitea_webhooks::Webhook) {
    let requester = payload.sender.email;
    let requested = payload.requested_reviewer.unwrap().email;

    let body = format!("{} requested a review from {}", requester, requested);
    let _ = send_slack_webhook(&body).await;
}

async fn reviewed(payload: gitea_webhooks::Webhook) {
    let reviewer = payload.sender.email;
    let review = payload.review.unwrap();

    let body = format!("{} {:?} the pull-request", reviewer, review.r#type);
    let _ = send_slack_webhook(&body).await;
}

async fn opened(payload: gitea_webhooks::Webhook) {
    let opener = payload.sender.email;
    let title = payload.pull_request.title;
    let body = payload.pull_request.body;
    let number = payload.pull_request.id;

    let body = format!("{} opened PR#{} \"{}\"\n{}", opener, number, title, body);
    let _ = send_slack_webhook(&body).await;
}
