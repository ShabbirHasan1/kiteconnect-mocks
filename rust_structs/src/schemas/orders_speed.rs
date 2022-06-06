use super::common::*;
use crate::utils::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orders {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OrdersData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrdersData {
    pub placed_by: String,
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_order_id: Option<String>,
    pub status: OrderStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message_raw: Option<String>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_timestamp: Option<NaiveDateTime>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_update_timestamp: Option<NaiveDateTime>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_timestamp: Option<NaiveDateTime>,
    pub variety: Variety,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<bool>,
    pub exchange: Exchanges,
    pub tradingsymbol: String,
    pub instrument_token: u64,
    pub order_type: OrderTypes,
    pub transaction_type: TransactionType,
    pub validity: Validity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_ttl: Option<i64>,
    pub product: Products,
    pub quantity: i64,
    pub disclosed_quantity: i64,
    pub price: f64,
    pub trigger_price: f64,
    pub average_price: f64,
    pub filled_quantity: i64,
    pub pending_quantity: i64,
    pub cancelled_quantity: i64,
    pub market_protection: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    pub guid: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::borrow::BorrowMut;
    // use chrono::NaiveDate;
    #[test]
    fn test_orders_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("orders.json").unwrap();
        // let mut jsonfile = std::fs::read_to_string("../orders.json").unwrap();
        // let deserialized: Orders = simd_json::from_reader(jsonfile)?;
        let deserialized: Orders = simd_json::from_reader(jsonfile)?;
        // let deserialized: Orders = Orders::read_from_file("../orders.json")?;
        // println!("{:#?}", &deserialized);
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        let serialized = simd_json::to_string(&deserialized).unwrap();
        println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }
}
