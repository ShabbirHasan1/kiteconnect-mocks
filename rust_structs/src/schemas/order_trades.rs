use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderTrades {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OrderTradesData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderTradesData {
    pub trade_id: String,
    pub order_id: String,
    pub exchange: Exchanges,
    pub tradingsymbol: String,
    pub instrument_token: u64,
    pub product: Products,
    pub average_price: f64,
    pub quantity: i64,
    pub exchange_order_id: String,
    pub transaction_type: TransactionType,
    pub fill_timestamp: String,
    pub order_timestamp: String,
    pub exchange_timestamp: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_order_trades_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_user_from_file("../order_trades.json").unwrap();
        let deserialized: OrderTrades = serde_json::from_reader(jsonfile)?;
        println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            OrderTrades {
                status: Status::Success,
                data: Some(vec![OrderTradesData {
                    trade_id: "10000000".to_owned(),
                    order_id: "200000000000000".to_owned(),
                    exchange: Exchanges::MCX,
                    tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                    instrument_token: 58424839,
                    product: Products::NRML,
                    average_price: 4852.0,
                    quantity: 1,
                    exchange_order_id: "300000000000000".to_owned(),
                    transaction_type: TransactionType::BUY,
                    fill_timestamp: "2021-05-31 16:00:36".to_owned(),
                    order_timestamp: "16:00:36".to_owned(),
                    exchange_timestamp: "2021-05-31 16:00:36".to_owned(),
                },]),
                ..OrderTrades::default()
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_order_trades_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: OrderTrades = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            OrderTrades {
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
