use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Positions {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PositionsData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionsData {
    pub net: Vec<PositionsDataDetails>,
    pub day: Vec<PositionsDataDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionsDataDetails {
    pub tradingsymbol: String,
    pub exchange: Exchanges,
    pub instrument_token: u32,
    pub product: Products,
    pub quantity: i64,
    pub overnight_quantity: i64,
    pub multiplier: i64,
    pub average_price: f64,
    pub close_price: f64,
    pub last_price: f64,
    pub value: f64,
    pub pnl: f64,
    #[serde(rename = "m2m")]
    pub mtm: f64,
    pub unrealised: f64,
    pub realised: f64,
    pub buy_quantity: i64,
    pub buy_price: f64,
    pub buy_value: f64,
    #[serde(rename = "buy_m2m")]
    pub buy_mtm: f64,
    pub sell_quantity: i64,
    pub sell_price: f64,
    pub sell_value: f64,
    #[serde(rename = "sell_m2m")]
    pub sell_mtm: f64,
    pub day_buy_quantity: i64,
    pub day_buy_price: f64,
    pub day_buy_value: f64,
    pub day_sell_quantity: i64,
    pub day_sell_price: f64,
    pub day_sell_value: f64,
}

#[test]
fn test_margins_json() -> serde_json::Result<()> {
    let jsonfile = crate::utils::read_user_from_file("../positions.json").unwrap();
    let deserialized: Positions = serde_json::from_reader(jsonfile)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        Positions {
            status: Status::Success,
            data: Some(PositionsData {
                net: vec![
                    PositionsDataDetails {
                        tradingsymbol: "LEADMINI17DECFUT".to_owned(),
                        exchange: Exchanges::MCX,
                        instrument_token: 53496327,
                        product: Products::NRML,
                        quantity: 1,
                        overnight_quantity: 0,
                        multiplier: 1000,
                        average_price: 161.05,
                        close_price: 0.0,
                        last_price: 161.05,
                        value: -161050.0,
                        pnl: 0.0,
                        mtm: 0.0,
                        unrealised: 0.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 161.05,
                        buy_value: 161050.0,
                        buy_mtm: 161050.0,
                        sell_quantity: 0,
                        sell_price: 0.0,
                        sell_value: 0.0,
                        sell_mtm: 0.0,
                        day_buy_quantity: 1,
                        day_buy_price: 161.05,
                        day_buy_value: 161050.0,
                        day_sell_quantity: 0,
                        day_sell_price: 0.0,
                        day_sell_value: 0.0,
                    },
                    PositionsDataDetails {
                        tradingsymbol: "GOLDGUINEA17DECFUT".to_owned(),
                        exchange: Exchanges::MCX,
                        instrument_token: 53505799,
                        product: Products::NRML,
                        quantity: 0,
                        overnight_quantity: 3,
                        multiplier: 1,
                        average_price: 0.0,
                        close_price: 23232.0,
                        last_price: 23355.0,
                        value: 801.0,
                        pnl: 801.0,
                        mtm: 276.0,
                        unrealised: 801.0,
                        realised: 0.0,
                        buy_quantity: 4,
                        buy_price: 23139.75,
                        buy_value: 92559.0,
                        buy_mtm: 93084.0,
                        sell_quantity: 4,
                        sell_price: 23340.0,
                        sell_value: 93360.0,
                        sell_mtm: 93360.0,
                        day_buy_quantity: 1,
                        day_buy_price: 23388.0,
                        day_buy_value: 23388.0,
                        day_sell_quantity: 4,
                        day_sell_price: 23340.0,
                        day_sell_value: 93360.0,
                    },
                    PositionsDataDetails {
                        tradingsymbol: "SBIN".to_owned(),
                        exchange: Exchanges::NSE,
                        instrument_token: 779521,
                        product: Products::CO,
                        quantity: 0,
                        overnight_quantity: 0,
                        multiplier: 1,
                        average_price: 0.0,
                        close_price: 0.0,
                        last_price: 308.4,
                        value: -2.0,
                        pnl: -2.0,
                        mtm: -2.0,
                        unrealised: -2.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 311.0,
                        buy_value: 311.0,
                        buy_mtm: 311.0,
                        sell_quantity: 1,
                        sell_price: 309.0,
                        sell_value: 309.0,
                        sell_mtm: 309.0,
                        day_buy_quantity: 1,
                        day_buy_price: 311.0,
                        day_buy_value: 311.0,
                        day_sell_quantity: 1,
                        day_sell_price: 309.0,
                        day_sell_value: 309.0,
                    },
                ]
                .to_vec(),
                day: vec![
                    PositionsDataDetails {
                        tradingsymbol: "GOLDGUINEA17DECFUT".to_owned(),
                        exchange: Exchanges::MCX,
                        instrument_token: 53505799,
                        product: Products::NRML,
                        quantity: -3,
                        overnight_quantity: 0,
                        multiplier: 1,
                        average_price: 23340.0,
                        close_price: 23232.0,
                        last_price: 23355.0,
                        value: 69972.0,
                        pnl: -93.0,
                        mtm: -93.0,
                        unrealised: -93.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 23388.0,
                        buy_value: 23388.0,
                        buy_mtm: 23388.0,
                        sell_quantity: 4,
                        sell_price: 23340.0,
                        sell_value: 93360.0,
                        sell_mtm: 93360.0,
                        day_buy_quantity: 1,
                        day_buy_price: 23388.0,
                        day_buy_value: 23388.0,
                        day_sell_quantity: 4,
                        day_sell_price: 23340.0,
                        day_sell_value: 93360.0,
                    },
                    PositionsDataDetails {
                        tradingsymbol: "LEADMINI17DECFUT".to_owned(),
                        exchange: Exchanges::MCX,
                        instrument_token: 53496327,
                        product: Products::NRML,
                        quantity: 1,
                        overnight_quantity: 0,
                        multiplier: 1000,
                        average_price: 161.05,
                        close_price: 0.0,
                        last_price: 161.05,
                        value: -161050.0,
                        pnl: 0.0,
                        mtm: 0.0,
                        unrealised: 0.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 161.05,
                        buy_value: 161050.0,
                        buy_mtm: 161050.0,
                        sell_quantity: 0,
                        sell_price: 0.0,
                        sell_value: 0.0,
                        sell_mtm: 0.0,
                        day_buy_quantity: 1,
                        day_buy_price: 161.05,
                        day_buy_value: 161050.0,
                        day_sell_quantity: 0,
                        day_sell_price: 0.0,
                        day_sell_value: 0.0,
                    },
                    PositionsDataDetails {
                        tradingsymbol: "SBIN".to_owned(),
                        exchange: Exchanges::NSE,
                        instrument_token: 779521,
                        product: Products::CO,
                        quantity: 0,
                        overnight_quantity: 0,
                        multiplier: 1,
                        average_price: 0.0,
                        close_price: 0.0,
                        last_price: 308.4,
                        value: -2.0,
                        pnl: -2.0,
                        mtm: -2.0,
                        unrealised: -2.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 311.0,
                        buy_value: 311.0,
                        buy_mtm: 311.0,
                        sell_quantity: 1,
                        sell_price: 309.0,
                        sell_value: 309.0,
                        sell_mtm: 309.0,
                        day_buy_quantity: 1,
                        day_buy_price: 311.0,
                        day_buy_value: 311.0,
                        day_sell_quantity: 1,
                        day_sell_price: 309.0,
                        day_sell_value: 309.0,
                    },
                ]
                .to_vec(),
            }),
            ..Positions::default()
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}

#[test]
fn test_positions_error() -> serde_json::Result<()> {
    let raw_data =
        r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
    let deserialized: Positions = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        Positions {
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
