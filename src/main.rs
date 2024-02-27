use axum::{
    Router,
    routing::post,
    extract::Json,
    };
use serde_json;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(post_handler));

    let listerner = tokio::net::TcpListener::bind("192.168.0.26:6969").await.unwrap();

    axum::serve(listerner, app).await.unwrap();
}


async fn post_handler(Json(payload): Json<serde_json::Value>) {
    let req_data = &payload["pull_request"];
    
    static mut COUNT: i32 = 0;

    if !req_data.is_null() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("C:/Users/aaron/Documents/intelli-hooks/src/inputs/tmp.json")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", serde_json::to_string_pretty(&payload).expect("oop")) {
            eprintln!("Couldn't write to file: {}", e);
        } else {
            unsafe {
                COUNT += 1;
                println!("Added object to file: {}", COUNT);
            }
        }

        if let Some(action) = payload["action"].as_str() {
            match action {
                "review_requested" => review_requested(&payload),
                "reviewed" => reviewed(&payload),
                "opened" => opened(&payload),
                _ => println!("Unhandled action \"{}\"", action),
            }
        }


    }
}

fn review_requested(payload: &serde_json::Value) {
println!("review requested");
}

fn reviewed(payload: &serde_json::Value) {
    println!("reviewed");
}

fn opened(payload: &serde_json::Value) {
    println!("opened");
}