use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderInfo {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OrderInfoData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderInfoData {
    pub average_price: f64,
    pub cancelled_quantity: i64,
    pub disclosed_quantity: i64,
    pub exchange: Exchanges,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_timestamp: Option<String>,
    pub filled_quantity: i64,
    pub instrument_token: u64,
    pub order_id: String,
    pub order_timestamp: String,
    pub order_type: OrderTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_order_id: Option<String>,
    pub pending_quantity: i64,
    pub placed_by: String,
    pub price: f64,
    pub product: Products,
    pub quantity: i64,
    pub status: OrderStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub tradingsymbol: String,
    pub transaction_type: TransactionType,
    pub trigger_price: f64,
    pub validity: Validity,
    pub variety: Variety,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_order_info_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_user_from_file("../order_info.json").unwrap();
        let deserialized: OrderInfo = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            OrderInfo {
                status: Status::Success,
                data: Some(vec![
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::NSE,
                        exchange_order_id: None,
                        exchange_timestamp: None,
                        filled_quantity: 0,
                        instrument_token: 1,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: "2017-12-29 11:06:52".to_owned(),
                        order_type: OrderTypes::LIMIT,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CNC,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::PUTORDERREQRECEIVED),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::BUY,
                        trigger_price: 0.0,
                        validity: Validity::DAY,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::NSE,
                        exchange_order_id: None,
                        exchange_timestamp: None,
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: "2017-12-29 11:06:52".to_owned(),
                        order_type: OrderTypes::LIMIT,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CNC,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::VALIDATIONPENDING),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::BUY,
                        trigger_price: 0.0,
                        validity: Validity::DAY,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::NSE,
                        exchange_order_id: None,
                        exchange_timestamp: None,
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: "2017-12-29 11:06:52".to_owned(),
                        order_type: OrderTypes::LIMIT,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CNC,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::OPENPENDING),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::BUY,
                        trigger_price: 0.0,
                        validity: Validity::DAY,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::NSE,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some("2017-12-29 11:06:52".to_owned()),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: "2017-12-29 11:06:52".to_owned(),
                        order_type: OrderTypes::LIMIT,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CNC,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::OPEN),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::BUY,
                        trigger_price: 0.0,
                        validity: Validity::DAY,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::NSE,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some("2017-12-29 11:06:52".to_owned()),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: "2017-12-29 11:08:16".to_owned(),
                        order_type: OrderTypes::LIMIT,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CNC,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::MODIFYVALIDATIONPENDING),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::BUY,
                        trigger_price: 0.0,
                        validity: Validity::DAY,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::NSE,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some("2017-12-29 11:06:52".to_owned()),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: "2017-12-29 11:08:16".to_owned(),
                        order_type: OrderTypes::LIMIT,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CNC,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::MODIFYPENDING),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::BUY,
                        trigger_price: 0.0,
                        validity: Validity::DAY,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::NSE,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some("2017-12-29 11:08:16".to_owned()),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: "2017-12-29 11:08:16".to_owned(),
                        order_type: OrderTypes::LIMIT,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CNC,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::MODIFIED),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::BUY,
                        trigger_price: 0.0,
                        validity: Validity::DAY,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::NSE,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some("2017-12-29 11:08:16".to_owned()),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: "2017-12-29 11:08:16".to_owned(),
                        order_type: OrderTypes::LIMIT,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.1,
                        product: Products::CNC,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::OPEN),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::BUY,
                        trigger_price: 0.0,
                        validity: Validity::DAY,
                        variety: Variety::Regular,
                    },
                ]),
                ..OrderInfo::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_order_info_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: OrderInfo = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            OrderInfo {
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
