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

    #[must_use]
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
    // use std::borrow::BorrowMut;
    use std::str::FromStr;
    #[test]
    fn test_postback_json_match_checksum() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../postback.json").unwrap();
        let deserialized: Postback = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert!(deserialized.match_checksum("0hdv7iw5examplesecret"));
        Ok(())
    }

    #[test]
    fn test_postback_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../postback.json").unwrap();
        let deserialized: Postback = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert!(deserialized.match_checksum("0hdv7iw5examplesecret"));
        assert_eq!(
            deserialized,
            Postback {
                user_id: "AB1234".to_owned(),
                unfilled_quantity: 0,
                app_id: 1234,
                checksum: Checksum::from_str(
                    "2011845d9348bd6795151bf4258102a03431e3bb12a79c0df73fcb4b7fde4b5d",
                )
                .unwrap(), // keeping api_secret as "0hdv7iw5examplesecret"
                placed_by: "AB1234".to_owned(),
                order_id: "220303000308932".to_owned(),
                exchange_order_id: Some("1000000001482421".to_owned()),
                parent_order_id: None,
                status: OrderStatus::Enum(OrderStatusValue::Complete),
                status_message: None,
                status_message_raw: None,
                order_timestamp: "2022-03-03 09:24:25".to_owned(),
                exchange_update_timestamp: "2022-03-03 09:24:25".to_owned(),
                exchange_timestamp: "2022-03-03 09:24:25".to_owned(),
                variety: Variety::Regular,
                exchange: Exchanges::Nse,
                tradingsymbol: "SBIN".to_owned(),
                instrument_token: 779521,
                order_type: OrderTypes::Market,
                transaction_type: TransactionType::Buy,
                validity: Validity::Day,
                product: Products::CashAndCarry,
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
}
