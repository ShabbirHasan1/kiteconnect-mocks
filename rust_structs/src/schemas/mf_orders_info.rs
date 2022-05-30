use super::common::*;
use crate::utils::*;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfOrdersInfo {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MfOrdersInfoData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfOrdersInfoData {
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
    #[test]
    fn test_mf_orders_info_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../mf_orders_info.json").unwrap();
        let deserialized: MfOrdersInfo = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfOrdersInfo {
                status: Status::Success,
                data: Some(MfOrdersInfoData {
                    status: Some(OrderStatus::Enum(OrderStatusValue::Open)),
                    purchase_type: Some(MfPurchaseType::Enum(MfPurchaseTypeValue::Fresh)),
                    folio: None,
                    order_timestamp: Some(NaiveDate::from_ymd(2021, 6, 29).and_hms(12, 20, 28)),
                    average_price: 0.0,
                    exchange_order_id: None,
                    last_price: 10.4324,
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
                },),
                ..MfOrdersInfo::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_mf_orders_info_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: MfOrdersInfo = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfOrdersInfo {
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
