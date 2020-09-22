use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn signup(username: String, password: String) -> std::result::Result<(), errors::ClientError> {
    
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct EventListItem {
    pub id: u32,
    pub title: String,
}

pub fn get_events() -> Vec<EventListItem> {
    match Client::new().get("http://localhost:8080/event").send() {
        Ok(mut res) => res.json().unwrap(),
        Err(err) => {
            panic!("Error while contacting server.");
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct EventInfoItemStock {
    pub id: u32,
    pub title: String,
    pub price: u8,
}

#[derive(Deserialize, Debug)]
pub struct EventInfoItem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub created: chrono::DateTime<chrono::prelude::Utc>,
    pub opens: chrono::DateTime<chrono::prelude::Utc>,
    pub closes: chrono::DateTime<chrono::prelude::Utc>,
    pub stocks: Vec<EventInfoItemStock>,
}

pub fn get_event_info(event_id: u32) -> EventInfoItem {
    let url = format!("http://localhost:8080/event/{}", event_id);
    match Client::new().get(&url).send() {
        Ok(mut res) => res.json().unwrap(),
        Err(err) => {
            panic!("Error while contacting server.");
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct StockInfoItemBin {
    pub price: u8,
    pub count: u32,
}

#[derive(Deserialize, Debug)]
pub struct StockInfoItem {
    pub id: u32,
    pub event_id: u32,
    pub title: String,
    pub price: u8,
    pub asks: Vec<StockInfoItemBin>,
    pub bids: Vec<StockInfoItemBin>,
}

pub fn get_stock_info(stock_id: u32) -> StockInfoItem {
    let url = format!("http://localhost:8080/stock/{}", stock_id);
    match Client::new().get(&url).send() {
        Ok(mut res) => res.json().unwrap(),
        Err(err) => {
            panic!("Error while contacting server.");
        }
    }
}

pub fn get_user_balance() -> u32 {
    let url = format!("http://localhost:8080/me/balance");
    match auth(Client::new().get(&url)).send() {
        Ok(mut res) => res.json().unwrap(),
        Err(err) => {
            panic!("Error while contacting server.");
        }
    }
}

pub fn buy(stock_id: u32, price: u8, count: u32) {
    #[derive(Serialize)]
    struct JSON {
        price: u8,
        count: u32,
    }
    let url = format!("http://localhost:8080/stock/{}/buy", stock_id);
    let data = JSON { price, count };
    match auth(Client::new().post(&url).json(&data)).send() {
        Ok(_) => (),
        Err(err) => {
            panic!("Error while contacting server.");
        }
    }
}

pub fn auth(req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    let token = std::env::var("PMARKET_TOKEN").unwrap_or("".to_string());
    req.header("Cookie", format!("SESSION-TOKEN={}", token))
}
