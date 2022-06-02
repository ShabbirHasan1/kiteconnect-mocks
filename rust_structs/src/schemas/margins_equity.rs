use super::common::{Exception, Status};
use super::margin::Equity;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginsEquity {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Equity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margins_equity_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../margins_equity.json").unwrap();
        let deserialized: MarginsEquity = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MarginsEquity {
                status: Status::Success,
                data: Some(Equity {
                    enabled: true,
                    net: 99725.05000000002,
                    available: crate::schemas::margin::Available {
                        adhoc_margin: 0.0,
                        cash: 245431.6,
                        opening_balance: 245431.6,
                        live_balance: 99725.05000000002,
                        collateral: 0.0,
                        intraday_payin: 0.0,
                    },
                    utilised: crate::schemas::margin::Utilised {
                        debits: 145706.55,
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
                }),
                ..MarginsEquity::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_margins_equity_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: MarginsEquity = serde_json::from_str(raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MarginsEquity {
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
