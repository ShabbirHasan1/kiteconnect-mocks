use super::common::*;
use crate::utils::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Holdings {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<HoldingsData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoldingsData {
    pub tradingsymbol: String,
    pub exchange: Exchanges,
    pub instrument_token: u64,
    pub isin: String,
    pub product: Products,
    pub price: f64,
    pub quantity: i64,
    pub used_quantity: i64,
    pub t1_quantity: i64,
    pub realised_quantity: i64,
    pub authorised_quantity: i64,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorised_date: Option<NaiveDateTime>,
    pub opening_quantity: i64,
    pub collateral_quantity: i64,
    #[serde(
        with = "serde_with::rust::string_empty_as_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub collateral_type: Option<String>,
    pub discrepancy: bool,
    pub average_price: f64,
    pub last_price: f64,
    pub close_price: f64,
    pub pnl: f64,
    pub day_change: f64,
    pub day_change_percentage: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_holdings_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../holdings.json").unwrap();
        let deserialized: Holdings = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Holdings {
                status: Status::Success,
                data: Some(vec![
                    HoldingsData {
                        tradingsymbol: "GOLDBEES".to_owned(),
                        exchange: Exchanges::Bse,
                        instrument_token: 151064324,
                        isin: "INF204KB17I5".to_owned(),
                        product: Products::CashAndCarry,
                        price: 0.0,
                        quantity: 2,
                        used_quantity: 0,
                        t1_quantity: 0,
                        realised_quantity: 2,
                        authorised_quantity: 0,
                        authorised_date: Some(NaiveDate::from_ymd(2021, 6, 8).and_hms(0, 0, 0)),
                        opening_quantity: 2,
                        collateral_quantity: 0,
                        collateral_type: None,
                        discrepancy: false,
                        average_price: 40.67,
                        last_price: 42.47,
                        close_price: 42.28,
                        pnl: 3.5999999999999943,
                        day_change: 0.18999999999999773,
                        day_change_percentage: 0.44938505203405327,
                    },
                    HoldingsData {
                        tradingsymbol: "IDEA".to_owned(),
                        exchange: Exchanges::Nse,
                        instrument_token: 3677697,
                        isin: "INE669E01016".to_owned(),
                        product: Products::CashAndCarry,
                        price: 0.0,
                        quantity: 5,
                        used_quantity: 0,
                        t1_quantity: 0,
                        realised_quantity: 5,
                        authorised_quantity: 0,
                        authorised_date: Some(NaiveDate::from_ymd(2021, 6, 8).and_hms(0, 0, 0)),
                        opening_quantity: 5,
                        collateral_quantity: 0,
                        collateral_type: None,
                        discrepancy: false,
                        average_price: 8.466,
                        last_price: 10.0,
                        close_price: 10.1,
                        pnl: 7.6700000000000035,
                        day_change: -0.09999999999999964,
                        day_change_percentage: -0.9900990099009866,
                    },
                ]),
                ..Holdings::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_holdings_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: Holdings = serde_json::from_str(raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Holdings {
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
