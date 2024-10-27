use std::cell::Cell;
use serde_json::Value;

pub async fn get_temperature() -> String {
    let tomorrow_url = Cell::new("api.tomorrow.io url");
    let response = reqwest::get(tomorrow_url.get()).await.unwrap().text().await.unwrap();
    let temperature = serde_json::from_str::<Value>(&response).unwrap()["data"]["values"]["temperature"].to_string();
    temperature
}

pub async fn get_cat_fact() -> String {
    let catfact_url = Cell::new("https://catfact.ninja/fact");
    let response = reqwest::get(catfact_url.get()).await.unwrap().text().await.unwrap();
    let cat_fact = serde_json::from_str::<Value>(&response).unwrap()["fact"].to_string();
    cat_fact
}