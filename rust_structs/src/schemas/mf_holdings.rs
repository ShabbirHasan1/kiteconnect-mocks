use super::common::*;
use crate::utils::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfHoldings {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MfHoldingsData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfHoldingsData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folio: Option<String>,
    pub fund: String,
    pub tradingsymbol: String,
    pub average_price: f64,
    pub last_price: f64,
    pub pnl: f64,
    #[serde(
        default,
        with = "double_optional_naive_date_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_price_date: Option<Option<NaiveDate>>,
    pub quantity: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_mf_holdings_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../mf_holdings.json").unwrap();
        let deserialized: MfHoldings = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfHoldings {
                status: Status::Success,
                data: Some(vec![
                    MfHoldingsData {
                        folio: Some("123123/123".into(),),
                        fund: "Kotak Select Focus Fund - Direct Plan".into(),
                        tradingsymbol: "INF174K01LS2".into(),
                        average_price: 30.729,
                        last_price: 33.014,
                        pnl: 594.769,
                        last_price_date: Some(Some(NaiveDate::from_ymd(2016, 11, 11))),
                        quantity: 260.337,
                    },
                    MfHoldingsData {
                        folio: Some("385080203".into(),),
                        fund: "DSP BlackRock Money Manager Fund".into(),
                        tradingsymbol: "INF740K01QQ3".into(),
                        average_price: 2146.131,
                        last_price: 2277.0708,
                        pnl: 61.018,
                        last_price_date: None,
                        quantity: 0.466,
                    },
                    MfHoldingsData {
                        folio: Some("1052046771".into(),),
                        fund: "HDFC TaxSaver - Regular Plan".into(),
                        tradingsymbol: "INF179K01BB8".into(),
                        average_price: 345.849,
                        last_price: 559.081,
                        pnl: 61963.074,
                        last_price_date: None,
                        quantity: 290.59,
                    },
                    MfHoldingsData {
                        folio: Some("91022348426".into(),),
                        fund: "Axis Long Term Equity Fund".into(),
                        tradingsymbol: "INF846K01131".into(),
                        average_price: 28.779,
                        last_price: 41.3876,
                        pnl: 44467.717,
                        last_price_date: None,
                        quantity: 3526.834,
                    },
                    MfHoldingsData {
                        folio: Some("488155267386".into(),),
                        fund: "Reliance Money Manager Fund".into(),
                        tradingsymbol: "INF204K01EY0".into(),
                        average_price: 1002.948,
                        last_price: 1007.5645,
                        pnl: 2.304,
                        last_price_date: None,
                        quantity: 0.499,
                    },
                ],),
                ..MfHoldings::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_mf_holdings_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: MfHoldings = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfHoldings {
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
