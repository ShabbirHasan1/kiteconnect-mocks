use super::common::{Exception, Exchanges, Products, Status, TransactionType};
use crate::utils::{optional_naive_date_time_from_str, optional_naive_time_from_str};
use chrono::{NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trades {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TradesData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradesData {
    pub trade_id: String,
    pub order_id: String,
    pub exchange: Exchanges,
    pub tradingsymbol: String,
    pub instrument_token: u64,
    pub product: Products,
    pub average_price: f64,
    pub quantity: i64,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_order_id: Option<String>,
    pub transaction_type: TransactionType,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub fill_timestamp: Option<NaiveDateTime>,
    #[serde(
        with = "optional_naive_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_timestamp: Option<NaiveTime>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_timestamp: Option<NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    use std::borrow::BorrowMut;
    #[test]
    fn test_trades_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../trades.json").unwrap();
        let deserialized: Trades = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Trades {
                status: Status::Success,
                data: Some(vec![
                    TradesData {
                        trade_id: "10000000".to_owned(),
                        order_id: "200000000000000".to_owned(),
                        exchange: Exchanges::Nse,
                        tradingsymbol: "SBIN".to_owned(),
                        instrument_token: 779521,
                        product: Products::CashAndCarry,
                        average_price: 420.65000000000003,
                        quantity: 1,
                        exchange_order_id: Some("300000000000000".to_owned()),
                        transaction_type: TransactionType::Buy,
                        fill_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(9, 16, 39)),
                        order_timestamp: Some(NaiveTime::from_hms(9, 16, 39)),
                        exchange_timestamp: Some(
                            NaiveDate::from_ymd(2021, 5, 31).and_hms(9, 16, 39)
                        ),
                    },
                    TradesData {
                        trade_id: "40000000".to_owned(),
                        order_id: "500000000000000".to_owned(),
                        exchange: Exchanges::Cds,
                        tradingsymbol: "USDINR21JUNFUT".to_owned(),
                        instrument_token: 412675,
                        product: Products::MarginIntradaySquareoff,
                        average_price: 72.755,
                        quantity: 1,
                        exchange_order_id: Some("600000000000000".to_owned()),
                        transaction_type: TransactionType::Buy,
                        fill_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(11, 18, 27)),
                        order_timestamp: Some(NaiveTime::from_hms(11, 18, 27)),
                        exchange_timestamp: Some(
                            NaiveDate::from_ymd(2021, 5, 31).and_hms(11, 18, 27)
                        ),
                    },
                    TradesData {
                        trade_id: "70000000".to_owned(),
                        order_id: "800000000000000".to_owned(),
                        exchange: Exchanges::Mcx,
                        tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                        instrument_token: 58424839,
                        product: Products::Normal,
                        average_price: 4852.0,
                        quantity: 1,
                        exchange_order_id: Some("312115100078593".to_owned()),
                        transaction_type: TransactionType::Buy,
                        fill_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 0, 36)),
                        order_timestamp: Some(NaiveTime::from_hms(16, 0, 36)),
                        exchange_timestamp: Some(
                            NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 0, 36)
                        ),
                    },
                    TradesData {
                        trade_id: "90000000".to_owned(),
                        order_id: "1100000000000000".to_owned(),
                        exchange: Exchanges::Mcx,
                        tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                        instrument_token: 58424839,
                        product: Products::Normal,
                        average_price: 4852.0,
                        quantity: 1,
                        exchange_order_id: Some("1200000000000000".to_owned()),
                        transaction_type: TransactionType::Buy,
                        fill_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 8, 41)),
                        order_timestamp: Some(NaiveTime::from_hms(16, 8, 41)),
                        exchange_timestamp: Some(
                            NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 8, 41)
                        ),
                    },
                ]),
                ..Trades::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_trades_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#
                .to_owned();
        let deserialized: Trades = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Trades {
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
