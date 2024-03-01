use serde_json;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn log_payload(payload: &serde_json::Value) {
    static mut COUNT: i32 = 0;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("C:/Users/aaron/Documents/intelli-hooks/src/inputs/tmp.json")
        .unwrap();

    if let Err(e) = writeln!(
        file,
        "{}",
        serde_json::to_string_pretty(&payload).expect("oop")
    ) {
        eprintln!("Couldn't write to file: {}", e);
    } else {
        unsafe {
            COUNT += 1;
            println!("Added object to file: {}", COUNT);
        }
    }
}
