use super::common::*;
use super::gtt_get_order::*;
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GttGetOrders {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GttGetOrderData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use std::borrow::BorrowMut;
    use chrono::NaiveDate;
    #[test]
    fn test_gtt_get_orders_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../gtt_get_orders.json").unwrap();
        let deserialized: GttGetOrders = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            GttGetOrders {
                status: Status::Success,
                data: Some(
                    vec![
                        GttGetOrderData {
                            id: 112127,
                            user_id: "XX0000".into(),
                            parent_trigger: None,
                            data_type: GttOrderType::Single,
                            created_at: Some(NaiveDate::from_ymd(2019, 9, 12).and_hms(13, 25, 16)),
                            updated_at: Some(NaiveDate::from_ymd(2019, 9, 12).and_hms(13, 25, 16)),
                            expires_at: Some(NaiveDate::from_ymd(2020, 9, 12).and_hms(13, 25, 16)),
                            status: GttOrderStatus::Active,
                            condition: GttCondition {
                                exchange: Exchanges::Nse,
                                last_price: 798.0,
                                tradingsymbol: "INFY".into(),
                                trigger_values: vec![
                                    702.0,
                                ],
                                instrument_token: 408065,
                            },
                            orders: vec![
                                GttOrder {
                                    exchange: Exchanges::Nse,
                                    tradingsymbol: "INFY".into(),
                                    product: Products::CashAndCarry,
                                    order_type: OrderTypes::Limit,
                                    transaction_type: TransactionType::Buy,
                                    quantity: 1,
                                    price: 702.5,
                                    result: None,
                                },
                            ],
                            meta: Some(Meta::HashMap(HashMap::new())),
                        },
                        GttGetOrderData {
                            id: 105099,
                            user_id: "XX0000".into(),
                            parent_trigger: None,
                            data_type: GttOrderType::TwoLeg,
                            created_at: Some(NaiveDate::from_ymd(2019, 9, 9).and_hms(15, 13, 22)),
                            updated_at: Some(NaiveDate::from_ymd(2019, 9, 9).and_hms(15, 15, 8)),
                            expires_at: Some(NaiveDate::from_ymd(2020, 1, 1).and_hms(12, 0, 0)),
                            status: GttOrderStatus::Triggered,
                            condition: GttCondition {
                                exchange: Exchanges::Nse,
                                last_price: 102.60000000000001,
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
                                                order_id: None,
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
                    ],
                ),
                ..GttGetOrders::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_gtt_get_orders_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#.to_owned();
        let deserialized: GttGetOrders = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            GttGetOrders {
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
