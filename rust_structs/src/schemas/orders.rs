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
    use chrono::NaiveDate;
    #[test]
    fn test_orders_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_user_from_file("../orders.json").unwrap();
        let deserialized: Orders = serde_json::from_reader(jsonfile)?;
        println!("{:#?}", &deserialized);
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
                            status: OrderStatus::Enum(OrderStatusValue::Cancelled),
                            status_message: None,
                            status_message_raw: None,
                            order_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(9, 18, 57)),
                            exchange_update_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(9, 18, 58)),
                            exchange_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(9, 15, 38)),
                            variety: Variety::Regular,
                            modified: None,
                            exchange: Exchanges::Cds,
                            tradingsymbol: "USDINR21JUNFUT".to_owned(),
                            instrument_token: 412675,
                            order_type: OrderTypes::Limit,
                            transaction_type: TransactionType::Buy,
                            validity: Validity::Day,
                            validity_ttl: None,
                            product: Products::Normal,
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
                            order_id: "700000000000000".to_owned(),
                            exchange_order_id: Some("800000000000000".to_owned()),
                            parent_order_id: None,
                            status: OrderStatus::Enum(OrderStatusValue::Complete),
                            status_message: None,
                            status_message_raw: None,
                            order_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 0, 36)),
                            exchange_update_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 0, 36)),
                            exchange_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 0, 36)),
                            variety: Variety::Regular,
                            modified: None,
                            exchange: Exchanges::Mcx,
                            tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                            instrument_token: 58424839,
                            order_type: OrderTypes::Limit,
                            transaction_type: TransactionType::Buy,
                            validity: Validity::Day,
                            validity_ttl: None,
                            product: Products::Normal,
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
                            tag: Some(Tags::String("connect test order1".to_owned())),
                            tags: Some(Tags::Vector(vec![
                                "connect test order1".to_owned()
                            ])),
                            guid: "XXXXXXX".to_owned(),
                            },
                        OrdersData {
                            placed_by: "XXXXXX".to_owned(),
                            order_id: "9000000000000000".to_owned(),
                            exchange_order_id: Some("1000000000000000".to_owned()),
                            parent_order_id: None,
                            status: OrderStatus::Enum(OrderStatusValue::Complete),
                            status_message: None,
                            status_message_raw: None,
                            order_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 8, 40)),
                            exchange_update_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 8, 41)),
                            exchange_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 8, 41)),
                            variety: Variety::Regular,
                            modified: None,
                            exchange: Exchanges::Mcx,
                            tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                            instrument_token: 58424839,
                            order_type: OrderTypes::Limit,
                            transaction_type: TransactionType::Buy,
                            validity: Validity::Day,
                            validity_ttl: None,
                            product: Products::Normal,
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
                            tag: Some(Tags::String("connect test order2".to_owned())),
                            tags: Some(Tags::Vector(vec![
                                "connect test order2".to_owned(),
                                "XXXXX".to_owned(),
                            ])),
                            guid: "XXXXXX".to_owned(),
                            },
                        OrdersData {
                            placed_by: "XXXXXX".to_owned(),
                            order_id: "220524001859672".to_owned(),
                            exchange_order_id: None,
                            parent_order_id: None,
                            status: OrderStatus::Enum(OrderStatusValue::Rejected),
                            status_message: Some("Insufficient funds. Required margin is 95417.84 but available margin is 74251.80. Check the orderbook for open orders.".to_owned()),
                            status_message_raw: Some("RMS:Margin Exceeds,Required:95417.84, Available:74251.80 for entity account-XXXXX across exchange across segment across product ".to_owned()),
                            order_timestamp: Some(NaiveDate::from_ymd(2022, 5, 24).and_hms(12, 26, 52)),
                            exchange_update_timestamp: None,
                            exchange_timestamp: None,
                            variety: Variety::Iceberg,
                            modified: Some(false),
                            exchange: Exchanges::Nse,
                            tradingsymbol: "SBIN".to_owned(),
                            instrument_token: 779521,
                            order_type: OrderTypes::Limit,
                            transaction_type: TransactionType::Buy,
                            validity: Validity::Ttl,
                            validity_ttl: Some(2),
                            product: Products::CashAndCarry,
                            quantity: 200,
                            disclosed_quantity: 0,
                            price: 463.0,
                            trigger_price: 0.0,
                            average_price: 0.0,
                            filled_quantity: 0,
                            pending_quantity: 0,
                            cancelled_quantity: 0,
                            market_protection: 0,
                            meta: Some(Meta::MetaIceberg(
                            MetaIceberg  {
                                iceberg: Iceberg {
                                leg: 1,
                                legs: 5,
                                leg_quantity: 200,
                                total_quantity: 1000,
                                remaining_quantity: 800
                                }
                            })),
                            tag: Some(Tags::Enum(TagsValue::IcebergOrder)),
                            tags: Some(Tags::Array([TagsValue::IcebergOrder])),
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
