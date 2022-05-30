use super::common::*;
use crate::utils::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GttGetOrder {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GttGetOrderData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GttGetOrderData {
    pub id: u64,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_trigger: Option<String>,
    #[serde(rename = "type")]
    pub data_type: GttOrderType,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<NaiveDateTime>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<NaiveDateTime>,
    pub status: GttOrderStatus,
    pub condition: GttCondition,
    pub orders: Vec<GttOrder>,
    pub meta: Option<Meta>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GttCondition {
    pub exchange: Exchanges,
    pub last_price: f64,
    pub tradingsymbol: String,
    pub trigger_values: Vec<f64>,
    pub instrument_token: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GttOrder {
    pub exchange: Exchanges,
    pub tradingsymbol: String,
    pub product: Products,
    pub order_type: OrderTypes,
    pub transaction_type: TransactionType,
    pub quantity: i64,
    pub price: f64,
    pub result: Option<GttResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GttResult {
    pub account_id: String,
    pub exchange: Exchanges,
    pub tradingsymbol: String,
    pub validity: Validity,
    pub product: Products,
    pub order_type: OrderTypes,
    pub transaction_type: TransactionType,
    pub quantity: i64,
    pub price: f64,
    #[serde(with = "serde_with::json::nested")]
    pub meta: Option<GttOrderResultMeta>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<NaiveDateTime>,
    pub triggered_at: f64,
    pub order_result: GttOrderResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GttOrderResult {
    pub status: Status,
    pub order_id: Option<String>,
    pub rejection_reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GttOrderResultMeta {
    pub app_id: u64,
    pub gtt: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_gtt_get_order_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../gtt_get_order.json").unwrap();
        let deserialized: GttGetOrder = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            GttGetOrder {
                status: Status::Success,
                data: Some(
                    GttGetOrderData {
                        id: 123,
                        user_id: "XX0000".into(),
                        parent_trigger: None,
                        data_type: GttOrderType::TwoLeg,
                        created_at: Some(NaiveDate::from_ymd(2019, 9, 9).and_hms(15, 13, 22)),
                        updated_at: Some(NaiveDate::from_ymd(2019, 9, 9).and_hms(15, 15, 8)),
                        expires_at: Some(NaiveDate::from_ymd(2020, 1, 1).and_hms(12, 0, 0)),
                        status: GttOrderStatus::Triggered,
                        condition: GttCondition {
                            exchange: Exchanges::Nse,
                            last_price: 102.6,
                            tradingsymbol: "RAIN".into(),
                            trigger_values: vec![
                                102.0,
                                103.7,
                            ],
                            instrument_token: 3926273,
                        },
                        orders: vec![
                            GttOrder {
                                exchange: Exchanges::Nse,
                                tradingsymbol: "RAIN".into(),
                                product: Products::CashAndCarry,
                                order_type: OrderTypes::Limit,
                                transaction_type: TransactionType::Sell,
                                quantity: 1,
                                price: 1.0,
                                result: None,
                            },
                            GttOrder {
                                exchange: Exchanges::Nse,
                                tradingsymbol: "RAIN".into(),
                                product: Products::CashAndCarry,
                                order_type: OrderTypes::Limit,
                                transaction_type: TransactionType::Sell,
                                quantity: 1,
                                price: 1.0,
                                result: Some(
                                    GttResult {
                                        account_id: "XX0000".into(),
                                        exchange: Exchanges::Nse,
                                        tradingsymbol: "RAIN".into(),
                                        validity: Validity::Day,
                                        product: Products::CashAndCarry,
                                        order_type: OrderTypes::Limit,
                                        transaction_type: TransactionType::Sell,
                                        quantity: 1,
                                        price: 1.0,
                                        meta: Some(
                                            GttOrderResultMeta {
                                                app_id: 12617,
                                                gtt: 105099,
                                            },
                                        ),
                                        timestamp: Some(NaiveDate::from_ymd(2019, 9, 9).and_hms(15, 15, 8)),
                                        triggered_at: 103.7,
                                        order_result: GttOrderResult {
                                            status: Status::Failed,
                                            order_id: Some(
                                                "".into(),
                                            ),
                                            rejection_reason: Some(
                                                "Your order price is lower than the current lower circuit limit of 70.65. Place an order within the daily range.".into(),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ],
                        meta: None,
                    },
                ),
                ..GttGetOrder::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_gtt_get_order_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: GttGetOrder = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            GttGetOrder {
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
