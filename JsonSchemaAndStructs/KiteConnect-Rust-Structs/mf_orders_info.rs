// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct MfOrdersInfo {
    pub data: Option<Data>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub amount: Option<i64>,
    pub average_price: Option<i64>,
    pub exchange_order_id: Option<serde_json::Value>,
    pub exchange_timestamp: Option<serde_json::Value>,
    pub folio: Option<serde_json::Value>,
    pub fund: Option<String>,
    pub last_price: Option<f64>,
    pub last_price_date: Option<String>,
    pub order_id: Option<String>,
    pub order_timestamp: Option<String>,
    pub placed_by: Option<String>,
    pub purchase_type: Option<String>,
    pub quantity: Option<i64>,
    pub settlement_id: Option<serde_json::Value>,
    pub status: Option<String>,
    pub status_message: Option<String>,
    pub tag: Option<serde_json::Value>,
    pub tradingsymbol: Option<String>,
    pub transaction_type: Option<String>,
    pub variety: Option<String>,
}
