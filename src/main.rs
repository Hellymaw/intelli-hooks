use std::{future::IntoFuture, thread};

use axum::{extract::Json, routing::post, Router};
use futures::join;
use gitea_webhooks::{Action, OutgoingWebhook, Review, User, Webhook};
use reqwest;
use serde_json;
use tokio::try_join;

pub mod gitea_webhooks;
pub mod slack_app;

const BIND_ADDRESS: &str = "192.168.0.26:6969";
const SLACK_WEBHOOK_ADDRESS: &str = "http://localhost:8000/";

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let app = Router::new().route("/", post(post_handler));
    let listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await.unwrap();

    let axum_handle = tokio::spawn(async {
        axum::serve(listener, app).await.unwrap();
    });

    let slack_handle = tokio::spawn(async {
        slack_app::run_slack_socket_mode().await.unwrap();
        panic!("The slack app shouldn't have finished, panic!");
    });

    let _ = try_join!(axum_handle, slack_handle);
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
    match serde_json::from_value::<Webhook>(payload) {
        Ok(webhook) => match webhook.action {
            Action::ReviewRequested {
                requested_reviewer: ref reviewer,
            } => review_requested(&webhook, &reviewer).await,
            Action::Reviewed { ref review } => reviewed(&webhook, &review).await,
            Action::Closed => opened(webhook).await,
            action => println!("Unhandled action \"{:?}\"", action),
        },
        Err(x) => println!("{}", x),
    }
}

async fn review_requested(payload: &Webhook, reviewer: &User) {
    let requester = &payload.sender.email;

    let body = format!("{} requested a review from {}", requester, reviewer.email);
    let _ = send_slack_webhook(&body).await;
}

async fn reviewed(payload: &Webhook, review: &Review) {
    let reviewer = &payload.sender.email;

    let body = format!("{} {:?} the pull-request", reviewer, review);
    let _ = send_slack_webhook(&body).await;
}

async fn opened(payload: Webhook) {
    let outgoing = OutgoingWebhook {
        email: payload.sender.email.to_owned(),
        title: "opened PR#".to_owned(),
        body: "".to_owned(),
    };

    let body = serde_json::to_string(&outgoing).unwrap();

    println!("{:?}", body);
    let _ = send_slack_webhook(&body).await;
}
