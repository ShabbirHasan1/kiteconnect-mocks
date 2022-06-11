use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Margins {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MarginsData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginsData {
    pub equity: Equity,
    pub commodity: Commodity,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equity {
    pub enabled: bool,
    pub net: f64,
    pub available: Available,
    pub utilised: Utilised,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commodity {
    pub enabled: bool,
    pub net: f64,
    pub available: Available,
    pub utilised: Utilised,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Available {
    pub adhoc_margin: f64,
    pub cash: f64,
    pub opening_balance: f64,
    pub live_balance: f64,
    pub collateral: f64,
    pub intraday_payin: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Utilised {
    pub debits: f64,
    pub exposure: f64,
    pub m2m_realised: f64,
    pub m2m_unrealised: f64,
    pub option_premium: f64,
    pub payout: f64,
    pub span: f64,
    pub holding_sales: f64,
    pub turnover: f64,
    pub liquid_collateral: f64,
    pub stock_collateral: f64,
    pub delivery: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    #[test]
    fn test_margins_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../margins.json").unwrap();
        let deserialized: Margins = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Margins {
                status: Status::Success,
                data: Some(MarginsData {
                    equity: Equity {
                        enabled: true,
                        net: 99725.05000000002,
                        available: Available {
                            adhoc_margin: 0.0,
                            cash: 245431.6,
                            opening_balance: 245431.6,
                            live_balance: 99725.05000000002,
                            collateral: 0.0,
                            intraday_payin: 0.0,
                        },
                        utilised: Utilised {
                            debits: 145706.55000000002,
                            exposure: 38981.25,
                            m2m_realised: 761.7,
                            m2m_unrealised: 0.0,
                            option_premium: 0.0,
                            payout: 0.0,
                            span: 101989.0,
                            holding_sales: 0.0,
                            turnover: 0.0,
                            liquid_collateral: 0.0,
                            stock_collateral: 0.0,
                            delivery: 0.0,
                        },
                    },
                    commodity: Commodity {
                        enabled: true,
                        net: 100661.70000000001,
                        available: Available {
                            adhoc_margin: 0.0,
                            cash: 100661.70000000001,
                            opening_balance: 100661.70000000001,
                            live_balance: 100661.70000000001,
                            collateral: 0.0,
                            intraday_payin: 0.0,
                        },
                        utilised: Utilised {
                            debits: 0.0,
                            exposure: 0.0,
                            m2m_realised: 0.0,
                            m2m_unrealised: 0.0,
                            option_premium: 0.0,
                            payout: 0.0,
                            span: 0.0,
                            holding_sales: 0.0,
                            turnover: 0.0,
                            liquid_collateral: 0.0,
                            stock_collateral: 0.0,
                            delivery: 0.0,
                        },
                    },
                }),
                ..Margins::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_margins_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#
                .to_owned();
        let deserialized: Margins = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Margins {
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
