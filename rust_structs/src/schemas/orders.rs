use super::common::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub order_timestamp: String,
    pub exchange_update_timestamp: String,
    pub exchange_timestamp: String,
    pub variety: Variety,
    pub exchange: Exchanges,
    pub tradingsymbol: String,
    pub instrument_token: u64,
    pub order_type: OrderTypes,
    pub transaction_type: TransactionType,
    pub validity: Validity,
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
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub guid: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_orders_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_user_from_file("../orders.json").unwrap();
        let deserialized: Orders = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Orders {
                status: Status::Success,
                data: Some(vec![
                    OrdersData {
                        placed_by: "XXXXXX".to_owned(),
                        order_id: "100000000000000".to_owned(),
                        exchange_order_id: Some("200000000000000".to_owned()),
                        parent_order_id: None,
                        status: OrderStatus::Enum(OrderStatusValue::CANCELLED),
                        status_message: None,
                        status_message_raw: None,
                        order_timestamp: "2021-05-31 09:18:57".to_owned(),
                        exchange_update_timestamp: "2021-05-31 09:18:58".to_owned(),
                        exchange_timestamp: "2021-05-31 09:15:38".to_owned(),
                        variety: Variety::Regular,
                        exchange: Exchanges::CDS,
                        tradingsymbol: "USDINR21JUNFUT".to_owned(),
                        instrument_token: 412675,
                        order_type: OrderTypes::LIMIT,
                        transaction_type: TransactionType::BUY,
                        validity: Validity::DAY,
                        product: Products::NRML,
                        quantity: 1,
                        disclosed_quantity: 0,
                        price: 72.0,
                        trigger_price: 0.0,
                        average_price: 0.0,
                        filled_quantity: 0,
                        pending_quantity: 1,
                        cancelled_quantity: 1,
                        market_protection: 0,
                        meta: Some(Meta::default()),
                        tag: None,
                        tags: None,
                        guid: "XXXXX".to_owned(),
                    },
                    OrdersData {
                        placed_by: "XXXXXX".to_owned(),
                        order_id: "300000000000000".to_owned(),
                        exchange_order_id: Some("400000000000000".to_owned()),
                        parent_order_id: None,
                        status: OrderStatus::Enum(OrderStatusValue::COMPLETE),
                        status_message: None,
                        status_message_raw: None,
                        order_timestamp: "2021-05-31 15:20:28".to_owned(),
                        exchange_update_timestamp: "2021-05-31 15:20:28".to_owned(),
                        exchange_timestamp: "2021-05-31 15:20:28".to_owned(),
                        variety: Variety::Regular,
                        exchange: Exchanges::NSE,
                        tradingsymbol: "IOC".to_owned(),
                        instrument_token: 415745,
                        order_type: OrderTypes::LIMIT,
                        transaction_type: TransactionType::BUY,
                        validity: Validity::DAY,
                        product: Products::CNC,
                        quantity: 1,
                        disclosed_quantity: 0,
                        price: 109.4,
                        trigger_price: 0.0,
                        average_price: 109.4,
                        filled_quantity: 1,
                        pending_quantity: 0,
                        cancelled_quantity: 0,
                        market_protection: 0,
                        meta: Some(Meta::default()),
                        tag: None,
                        tags: None,
                        guid: "XXXXXX".to_owned(),
                    },
                    OrdersData {
                        placed_by: "XXXXXX".to_owned(),
                        order_id: "500000000000000".to_owned(),
                        exchange_order_id: Some("600000000000000".to_owned()),
                        parent_order_id: None,
                        status: OrderStatus::Enum(OrderStatusValue::COMPLETE),
                        status_message: None,
                        status_message_raw: None,
                        order_timestamp: "2021-05-31 15:20:51".to_owned(),
                        exchange_update_timestamp: "2021-05-31 15:20:52".to_owned(),
                        exchange_timestamp: "2021-05-31 15:20:52".to_owned(),
                        variety: Variety::Regular,
                        exchange: Exchanges::NSE,
                        tradingsymbol: "IOC".to_owned(),
                        instrument_token: 415745,
                        order_type: OrderTypes::MARKET,
                        transaction_type: TransactionType::SELL,
                        validity: Validity::DAY,
                        product: Products::CNC,
                        quantity: 1,
                        disclosed_quantity: 0,
                        price: 0.0,
                        trigger_price: 0.0,
                        average_price: 109.35,
                        filled_quantity: 1,
                        pending_quantity: 0,
                        cancelled_quantity: 0,
                        market_protection: 0,
                        meta: Some(Meta::default()),
                        tag: None,
                        tags: None,
                        guid: "XXXX".to_owned(),
                    },
                    OrdersData {
                        placed_by: "XXXXXX".to_owned(),
                        order_id: "700000000000000".to_owned(),
                        exchange_order_id: Some("800000000000000".to_owned()),
                        parent_order_id: None,
                        status: OrderStatus::Enum(OrderStatusValue::COMPLETE),
                        status_message: None,
                        status_message_raw: None,
                        order_timestamp: "2021-05-31 16:00:36".to_owned(),
                        exchange_update_timestamp: "2021-05-31 16:00:36".to_owned(),
                        exchange_timestamp: "2021-05-31 16:00:36".to_owned(),
                        variety: Variety::Regular,
                        exchange: Exchanges::MCX,
                        tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                        instrument_token: 58424839,
                        order_type: OrderTypes::LIMIT,
                        transaction_type: TransactionType::BUY,
                        validity: Validity::DAY,
                        product: Products::NRML,
                        quantity: 1,
                        disclosed_quantity: 0,
                        price: 4854.0,
                        trigger_price: 0.0,
                        average_price: 4852.0,
                        filled_quantity: 1,
                        pending_quantity: 0,
                        cancelled_quantity: 0,
                        market_protection: 0,
                        meta: Some(Meta::default()),
                        tag: Some("connect test order1".to_owned()),
                        tags: Some(vec!["connect test order1".to_owned(),]),
                        guid: "XXXXXXX".to_owned(),
                    },
                    OrdersData {
                        placed_by: "XXXXXX".to_owned(),
                        order_id: "9000000000000000".to_owned(),
                        exchange_order_id: Some("1000000000000000".to_owned()),
                        parent_order_id: None,
                        status: OrderStatus::Enum(OrderStatusValue::COMPLETE),
                        status_message: None,
                        status_message_raw: None,
                        order_timestamp: "2021-05-31 16:08:40".to_owned(),
                        exchange_update_timestamp: "2021-05-31 16:08:41".to_owned(),
                        exchange_timestamp: "2021-05-31 16:08:41".to_owned(),
                        variety: Variety::Regular,
                        exchange: Exchanges::MCX,
                        tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                        instrument_token: 58424839,
                        order_type: OrderTypes::LIMIT,
                        transaction_type: TransactionType::BUY,
                        validity: Validity::DAY,
                        product: Products::NRML,
                        quantity: 1,
                        disclosed_quantity: 0,
                        price: 4854.0,
                        trigger_price: 0.0,
                        average_price: 4852.0,
                        filled_quantity: 1,
                        pending_quantity: 0,
                        cancelled_quantity: 0,
                        market_protection: 0,
                        meta: Some(Meta::default()),
                        tag: Some("connect test order2".to_owned()),
                        tags: Some(vec!["connect test order2".to_owned(), "XXXXX".to_owned(),]),
                        guid: "XXXXXX".to_owned(),
                    },
                ]),
                ..Orders::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_orders_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: Orders = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Orders {
                status: Status::Error,
                data: None,
                message: Some("Error message".to_owned()),
                error_type: Some(Exception::GeneralException),
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }
}
