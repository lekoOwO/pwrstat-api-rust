#[macro_use] extern crate rocket;

use std::process::Command;
use std::collections::HashMap;
use rocket::serde::json::Json;

#[get("/pwrstat")]
fn pwrstat() -> Json<HashMap<String, String>> {
    let output = Command::new("pwrstat")
        .arg("-status")
        .output()
        .expect("Failed to execute pwrstat command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut status_map = HashMap::new();

    for line in stdout.lines() {
        let line = line.trim();
        let line = line.replace(". ", ";").replace(".", "");
        let parts: Vec<&str> = line.split(';').collect();

        if parts.len() > 1 {
            status_map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    Json(status_map)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![pwrstat])
}
