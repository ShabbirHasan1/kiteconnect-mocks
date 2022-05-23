use super::common::{Exception, Status};
use super::margin::Commodity;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginsCommodity {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Commodity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[test]
fn test_margins_commodity_json() -> serde_json::Result<()> {
    let jsonfile = crate::utils::read_user_from_file("../margin_commodity.json").unwrap();
    let deserialized: MarginsCommodity = serde_json::from_reader(jsonfile)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        MarginsCommodity {
            status: Status::Success,
            data: Some(Commodity {
                enabled: true,
                net: 96426.33,
                available: super::margin::Available {
                    adhoc_margin: 0.0,
                    cash: 97297.5,
                    opening_balance: 97297.5,
                    live_balance: 96426.32999999999,
                    collateral: 0.0,
                    intraday_payin: 0.0,
                },
                utilised: super::margin::Utilised {
                    debits: 871.17,
                    exposure: 117.35,
                    m2m_realised: 0.0,
                    m2m_unrealised: 0.0,
                    option_premium: 0.0,
                    payout: -0.0,
                    span: 563.0,
                    holding_sales: 0.0,
                    turnover: 0.0,
                    liquid_collateral: 0.0,
                    stock_collateral: 0.0,
                    delivery: 0.0,
                },
            }),
            ..MarginsCommodity::default()
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}

#[test]
fn test_margins_commodity_error() -> serde_json::Result<()> {
    let raw_data =
        r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
    let deserialized: MarginsCommodity = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        MarginsCommodity {
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
