use super::common::*;
use crate::utils::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MfInstruments {
    tradingsymbol: String,
    amc: String,
    purchase_allowed: u8,
    redemption_allowed: u8,
    minimum_purchase_amount: f64,
    purchase_amount_multiplier: f64,
    minimum_additional_purchase_amount: f64,
    minimum_redemption_quantity: f64,
    redemption_quantity_multiplier: f64,
    dividend_type: DividendType,
    scheme_type: MfScheme,
    plan: MfPlan,
    settlement_type: SettlementType,
    last_price: f64,
    #[serde(
        with = "optional_naive_date_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    last_price_date: Option<NaiveDate>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    use chrono::NaiveDate;
    use csv::Reader;
    #[test]
    fn test_mf_instruments_csv() -> Result<(), Box<dyn std::error::Error>> {
        let csvfile = Reader::from_path("../mf_instruments.csv").unwrap();
        let mut deserialized = csvfile.into_deserialize::<MfInstruments>();
        if let Some(result) = deserialized.next() {
            let instrument_record: MfInstruments = result?;
            // println!("{:#?}", instrument_record);
            assert_eq!(
                instrument_record,
                MfInstruments {
                    tradingsymbol: "INF209K01157".into(),
                    amc: "BirlaSunLifeMutualFund_MF".into(),
                    purchase_allowed: 1,
                    redemption_allowed: 1,
                    minimum_purchase_amount: 1000.0,
                    purchase_amount_multiplier: 1.0,
                    minimum_additional_purchase_amount: 1000.0,
                    minimum_redemption_quantity: 0.001,
                    redemption_quantity_multiplier: 0.001,
                    dividend_type: DividendType::Payout,
                    scheme_type: MfScheme::Equity,
                    plan: MfPlan::Regular,
                    settlement_type: SettlementType::Tthree,
                    last_price: 106.8,
                    last_price_date: Some(NaiveDate::from_ymd(2017, 11, 23)),
                }
            );
            // for result in deserialized {
            //     let instrument_record: MfInstruments = result?;
            //     println!("{:#?}", instrument_record);
            // }
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }
}
