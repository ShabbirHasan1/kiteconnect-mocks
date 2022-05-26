use super::checksum::Checksum;
use super::common::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Postback {
    pub user_id: String,
    pub unfilled_quantity: i64,
    pub app_id: i64,
    pub checksum: Checksum,
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
    pub order_timestamp: String,
    pub exchange_update_timestamp: String,
    pub exchange_timestamp: String,
    pub variety: Variety,
    pub exchange: Exchanges,
    pub tradingsymbol: String,
    pub instrument_token: i64,
    pub order_type: OrderTypes,
    pub transaction_type: TransactionType,
    pub validity: Validity,
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
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub guid: String,
}

impl Postback {
    pub fn calculate_checksum<T: Into<String>>(&self, api_secret: T) -> Checksum {
        let to_be_hashed = format!(
            "{}{}{}",
            self.order_id,
            self.order_timestamp,
            api_secret.into()
        );
        let hash = Sha256::new()
            .chain_update(to_be_hashed.as_bytes())
            .finalize();
        Checksum::Sha256(hash.as_slice().try_into().unwrap())
    }

    pub fn set_checksum<T: Into<String>>(mut self, api_secret: T) -> Self {
        self.checksum = self.calculate_checksum(api_secret);
        self
    }

    pub fn match_checksum<T: Into<String>>(&self, api_secret: T) -> bool {
        self.checksum == self.calculate_checksum(api_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn test_postback_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_user_from_file("../postback.json").unwrap();
        let deserialized: Postback = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert!(deserialized.match_checksum("API_SECRET"));
        assert_eq!(
            deserialized,
            Postback {
                user_id: "AB1234".to_owned(),
                unfilled_quantity: 0,
                app_id: 1234,
                checksum: Checksum::from_str(
                    "aa389e3f151b51d34809d96299463207da0353105274d9495bad54a38ec227c1",
                )
                .unwrap(), // keeping api_secret as "API_SECRET"
                placed_by: "AB1234".to_owned(),
                order_id: "220303000308932".to_owned(),
                exchange_order_id: Some("1000000001482421".to_owned()),
                parent_order_id: None,
                status: OrderStatus::Enum(OrderStatusValue::COMPLETE),
                status_message: None,
                status_message_raw: None,
                order_timestamp: "2022-03-03 09:24:25".to_owned(),
                exchange_update_timestamp: "2022-03-03 09:24:25".to_owned(),
                exchange_timestamp: "2022-03-03 09:24:25".to_owned(),
                variety: Variety::Regular,
                exchange: Exchanges::NSE,
                tradingsymbol: "SBIN".to_owned(),
                instrument_token: 779521,
                order_type: OrderTypes::MARKET,
                transaction_type: TransactionType::BUY,
                validity: Validity::DAY,
                product: Products::CNC,
                quantity: 1,
                disclosed_quantity: 0,
                price: 0.0,
                trigger_price: 0.0,
                average_price: 470.0,
                filled_quantity: 1,
                pending_quantity: 0,
                cancelled_quantity: 0,
                market_protection: 0,
                meta: Some(Meta::default()),
                tag: None,
                tags: None,
                guid: "XXXXXX".to_owned(),
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_postback_json_match_checksum() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_user_from_file("../postback.json").unwrap();
        let deserialized: Postback = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert!(deserialized.match_checksum("API_SECRET"));
        Ok(())
    }
}
