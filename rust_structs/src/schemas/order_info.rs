use super::common::*;
use crate::utils::*;
use chrono::NaiveDateTime;
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
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_timestamp: Option<NaiveDateTime>,
    pub filled_quantity: i64,
    pub instrument_token: u64,
    pub order_id: String,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_timestamp: Option<NaiveDateTime>,
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
    use chrono::NaiveDate;
    #[test]
    fn test_order_info_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../order_info.json").unwrap();
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
                        exchange: Exchanges::Nse,
                        exchange_order_id: None,
                        exchange_timestamp: None,
                        filled_quantity: 0,
                        instrument_token: 1,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 6, 52)),
                        order_type: OrderTypes::Limit,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CashAndCarry,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::PutOrderReqReceived),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        validity: Validity::Day,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::Nse,
                        exchange_order_id: None,
                        exchange_timestamp: None,
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 6, 52)),
                        order_type: OrderTypes::Limit,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CashAndCarry,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::ValidationPending),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        validity: Validity::Day,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::Nse,
                        exchange_order_id: None,
                        exchange_timestamp: None,
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 6, 52)),
                        order_type: OrderTypes::Limit,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CashAndCarry,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::OpenPending),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        validity: Validity::Day,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::Nse,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some(
                            NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 6, 52)
                        ),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 6, 52)),
                        order_type: OrderTypes::Limit,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CashAndCarry,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::Open),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        validity: Validity::Day,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::Nse,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some(
                            NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 6, 52)
                        ),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 8, 16)),
                        order_type: OrderTypes::Limit,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CashAndCarry,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::ModifyValidationPending),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        validity: Validity::Day,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::Nse,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some(
                            NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 6, 52)
                        ),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 8, 16)),
                        order_type: OrderTypes::Limit,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CashAndCarry,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::ModifyPending),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        validity: Validity::Day,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::Nse,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some(
                            NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 8, 16)
                        ),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 8, 16)),
                        order_type: OrderTypes::Limit,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.0,
                        product: Products::CashAndCarry,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::Modified),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        validity: Validity::Day,
                        variety: Variety::Regular,
                    },
                    OrderInfoData {
                        average_price: 0.0,
                        cancelled_quantity: 0,
                        disclosed_quantity: 0,
                        exchange: Exchanges::Nse,
                        exchange_order_id: Some("1300000001887410".to_owned()),
                        exchange_timestamp: Some(
                            NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 8, 16)
                        ),
                        filled_quantity: 0,
                        instrument_token: 779521,
                        order_id: "171229000724687".to_owned(),
                        order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 8, 16)),
                        order_type: OrderTypes::Limit,
                        parent_order_id: None,
                        pending_quantity: 1,
                        placed_by: "DA0017".to_owned(),
                        price: 300.1,
                        product: Products::CashAndCarry,
                        quantity: 1,
                        status: OrderStatus::Enum(OrderStatusValue::Open),
                        status_message: None,
                        tag: None,
                        tags: None,
                        tradingsymbol: "SBIN".to_owned(),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        validity: Validity::Day,
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
        let deserialized: OrderInfo = serde_json::from_str(raw_data)?;
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
