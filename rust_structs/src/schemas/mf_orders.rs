use super::common::*;
use crate::utils::*;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfOrders {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MfOrdersData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfOrdersData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<MfPurchaseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folio: Option<String>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_timestamp: Option<NaiveDateTime>,
    pub average_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_order_id: Option<String>,
    pub last_price: f64,
    pub tradingsymbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_id: Option<String>,
    pub transaction_type: TransactionType,
    pub order_id: String,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub placed_by: String,
    #[serde(
        with = "optional_naive_date_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_timestamp: Option<NaiveDate>,
    pub variety: Variety,
    #[serde(
        with = "optional_naive_date_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_price_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    pub fund: String,
    pub quantity: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::borrow::BorrowMut;
    #[test]
    fn test_mf_orders_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../mf_orders.json").unwrap();
        let deserialized: MfOrders = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfOrders {
                status: Status::Success,
                data: Some(vec![
                    MfOrdersData {
                        status: Some(OrderStatus::Enum(OrderStatusValue::Rejected),),
                        purchase_type: Some(MfPurchaseType::Enum(MfPurchaseTypeValue::Fresh),),
                        folio: None,
                        order_timestamp: Some(NaiveDate::from_ymd(2021, 6, 30).and_hms(8, 33, 7)),
                        average_price: 0.0,
                        exchange_order_id: Some("254657127".into(),),
                        last_price: 30.68,
                        tradingsymbol: "INF179K01VY8".into(),
                        settlement_id: Some("2122061".into(),),
                        transaction_type: TransactionType::Buy,
                        order_id: "271989e0-a64e-4cf3-b4e4-afb8f38dd203".into(),
                        amount: 1000.0,
                        tag: None,
                        placed_by: "ZV8062".into(),
                        exchange_timestamp: Some(NaiveDate::from_ymd(2021, 6, 30)),
                        variety: Variety::AmcSip,
                        last_price_date: Some(NaiveDate::from_ymd(2021, 6, 29)),
                        status_message: Some("AMC SIP: Insufficient balance.".into(),),
                        fund: "HDFC Balanced Advantage Fund - Direct Plan".into(),
                        quantity: 0.0,
                    },
                    MfOrdersData {
                        status: Some(OrderStatus::Enum(OrderStatusValue::Rejected),),
                        purchase_type: Some(MfPurchaseType::Enum(MfPurchaseTypeValue::Additional),),
                        folio: None,
                        order_timestamp: Some(NaiveDate::from_ymd(2021, 6, 30).and_hms(1, 30, 2)),
                        average_price: 0.0,
                        exchange_order_id: None,
                        last_price: 52.798,
                        tradingsymbol: "INF174K01LS2".into(),
                        settlement_id: None,
                        transaction_type: TransactionType::Buy,
                        order_id: "ef7e696c-2fa6-400b-b180-eb25e6a04ccf".into(),
                        amount: 2000.0,
                        tag: Some("coinandroidsip".into(),),
                        placed_by: "ZV8062".into(),
                        exchange_timestamp: None,
                        variety: Variety::Sip,
                        last_price_date: Some(NaiveDate::from_ymd(2021, 6, 29)),
                        status_message: Some("SIP: Insufficient balance.".into(),),
                        fund: "Kotak Flexicap Fund - Direct Plan".into(),
                        quantity: 0.0,
                    },
                    MfOrdersData {
                        status: Some(OrderStatus::Enum(OrderStatusValue::Open),),
                        purchase_type: Some(MfPurchaseType::Enum(MfPurchaseTypeValue::Fresh),),
                        folio: None,
                        order_timestamp: Some(NaiveDate::from_ymd(2021, 6, 29).and_hms(12, 20, 28)),
                        average_price: 0.0,
                        exchange_order_id: None,
                        last_price: 10.432400000000001,
                        tradingsymbol: "INF761K01EE1".into(),
                        settlement_id: None,
                        transaction_type: TransactionType::Buy,
                        order_id: "2b6ad4b7-c84e-4c76-b459-f3a8994184f1".into(),
                        amount: 5000.0,
                        tag: None,
                        placed_by: "ZV8062".into(),
                        exchange_timestamp: None,
                        variety: Variety::Regular,
                        last_price_date: Some(NaiveDate::from_ymd(2021, 6, 29)),
                        status_message: Some("Insufficient fund. 1/5".into(),),
                        fund: "BOI AXA Arbitrage Fund - Direct Plan".into(),
                        quantity: 0.0,
                    },
                    MfOrdersData {
                        status: Some(OrderStatus::Enum(OrderStatusValue::Rejected),),
                        purchase_type: Some(MfPurchaseType::Enum(MfPurchaseTypeValue::Fresh),),
                        folio: None,
                        order_timestamp: Some(NaiveDate::from_ymd(2021, 6, 29).and_hms(8, 36, 41)),
                        average_price: 0.0,
                        exchange_order_id: Some("254447867".into(),),
                        last_price: 271.75,
                        tradingsymbol: "INF179K01WA6".into(),
                        settlement_id: Some("2122060".into(),),
                        transaction_type: TransactionType::Buy,
                        order_id: "40410882-b1f8-4938-bb08-4bef2765cbfb".into(),
                        amount: 1000.0,
                        tag: None,
                        placed_by: "ZV8062".into(),
                        exchange_timestamp: Some(NaiveDate::from_ymd(2021, 6, 29)),
                        variety: Variety::AmcSip,
                        last_price_date: Some(NaiveDate::from_ymd(2021, 6, 29)),
                        status_message: Some("AMC SIP: Insufficient balance.".into(),),
                        fund: "HDFC Balanced Advantage Fund - Direct Plan".into(),
                        quantity: 0.0,
                    },
                    MfOrdersData {
                        status: Some(OrderStatus::Enum(OrderStatusValue::Open),),
                        purchase_type: Some(MfPurchaseType::Enum(MfPurchaseTypeValue::Fresh),),
                        folio: None,
                        order_timestamp: Some(NaiveDate::from_ymd(2021, 6, 24).and_hms(15, 37, 27)),
                        average_price: 0.0,
                        exchange_order_id: None,
                        last_price: 11.5182,
                        tradingsymbol: "INF109K01V59".into(),
                        settlement_id: None,
                        transaction_type: TransactionType::Buy,
                        order_id: "e67b8741-5054-4fd5-a2da-8c672e1f494a".into(),
                        amount: 5000.0,
                        tag: None,
                        placed_by: "ZV8062".into(),
                        exchange_timestamp: None,
                        variety: Variety::Regular,
                        last_price_date: Some(NaiveDate::from_ymd(2021, 6, 29)),
                        status_message: Some("Insufficient fund. 3/5".into(),),
                        fund: "ICICI Prudential Bond Fund - Direct Plan".into(),
                        quantity: 0.0,
                    },
                ],),
                ..MfOrders::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_mf_orders_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#
                .to_owned();
        let deserialized: MfOrders = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfOrders {
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
