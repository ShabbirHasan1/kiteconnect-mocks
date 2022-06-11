use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasketMargins {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BasketMarginsData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasketMarginsData {
    #[serde(rename = "initial")]
    pub initial_margin: Margin,
    #[serde(rename = "final")]
    pub final_margin: Margin,
    pub orders: Vec<Margin>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Margin {
    #[serde(rename = "type")]
    pub order_margins_type: OptionalOrderMarginTypes, //equity/commodity
    pub tradingsymbol: String,       // Trading symbol of the instrument
    pub exchange: OptionalExchanges, // Name of the exchange
    pub span: f64,                   // SPAN margins
    pub exposure: f64,               // Exposure margins
    pub option_premium: f64,         // Option premium
    pub additional: f64,             // Additional margins
    pub bo: f64,                     // Cash credit
    pub cash: f64,                   // Cash credit
    pub var: f64,                    // VAR
    pub pnl: MarginPnl,              // realised and unrealised profit and loss
    pub total: f64,                  // Total margin block
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginPnl {
    pub realised: f64,
    pub unrealised: f64,
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::*;
    #[test]
    fn test_basket_margins_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../basket_margins.json").unwrap();
        let deserialized: BasketMargins = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            BasketMargins {
                status: Status::Success,
                data: Some(BasketMarginsData {
                    initial_margin: Margin {
                        order_margins_type: OptionalOrderMarginTypes::String("".to_owned()),
                        tradingsymbol: "".to_owned(),
                        exchange: OptionalExchanges::String("".to_owned()),
                        span: 0.0,
                        exposure: 36475.7085,
                        option_premium: 4728.75,
                        additional: 0.0,
                        bo: 0.0,
                        cash: 0.0,
                        var: 0.0,
                        pnl: MarginPnl {
                            realised: 0.0,
                            unrealised: 0.0,
                        },
                        total: 41204.4585,
                    },
                    final_margin: Margin {
                        order_margins_type: OptionalOrderMarginTypes::String("".to_owned()),
                        tradingsymbol: "".to_owned(),
                        exchange: OptionalExchanges::String("".to_owned()),
                        span: 1.8189894035458563e-12,
                        exposure: 36475.7085,
                        option_premium: 4591.875,
                        additional: 0.0,
                        bo: 0.0,
                        cash: 0.0,
                        var: 0.0,
                        pnl: MarginPnl {
                            realised: 0.0,
                            unrealised: 0.0,
                        },
                        total: 41067.5835,
                    },
                    orders: vec![
                        Margin {
                            order_margins_type: OptionalOrderMarginTypes::Enum(
                                OrderMarginTypes::Equity
                            ),
                            tradingsymbol: "NIFTY21JUL15400PE".to_owned(),
                            exchange: OptionalExchanges::Enum(Exchanges::Nfo),
                            span: 0.0,
                            exposure: 0.0,
                            option_premium: 4728.75,
                            additional: 0.0,
                            bo: 0.0,
                            cash: 0.0,
                            var: 0.0,
                            pnl: MarginPnl {
                                realised: 0.0,
                                unrealised: 0.0,
                            },
                            total: 4728.75,
                        },
                        Margin {
                            order_margins_type: OptionalOrderMarginTypes::Enum(
                                OrderMarginTypes::Equity
                            ),
                            tradingsymbol: "NIFTY21JUL14450PE".to_owned(),
                            exchange: OptionalExchanges::Enum(Exchanges::Nfo),
                            span: 0.0,
                            exposure: 36475.7085,
                            option_premium: 0.0,
                            additional: 0.0,
                            bo: 0.0,
                            cash: 0.0,
                            var: 0.0,
                            pnl: MarginPnl {
                                realised: 0.0,
                                unrealised: 0.0,
                            },
                            total: 36475.7085,
                        },
                    ],
                }),
                ..BasketMargins::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_basket_margins_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#
                .to_owned();
        let deserialized: BasketMargins = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            BasketMargins {
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
