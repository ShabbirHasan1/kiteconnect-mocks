use super::common::*;
use crate::utils::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Instruments {
    instrument_token: u64,
    exchange_token: u64,
    tradingsymbol: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    name: Option<String>,
    last_price: f64,
    expiry: Option<Expiry>,
    strike: f64,
    tick_size: f64,
    lot_size: u64,
    instrument_type: InstrumentType,
    segment: Segment,
    exchange: Exchanges,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expiry {
    #[serde(with = "naive_date_from_str")]
    NaiveDate(NaiveDate),
    String(String),
}

impl Default for Expiry {
    fn default() -> Self {
        Self::NaiveDate(NaiveDate::from_ymd(0, 0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use chrono::NaiveDate;
    use csv::Reader;
    #[test]
    fn test_instruments_nse() -> Result<(), Box<dyn std::error::Error>> {
        let csvfile = Reader::from_path("../instruments_nse.csv").unwrap();
        let mut deserialized = csvfile.into_deserialize::<Instruments>();
        if let Some(result) = deserialized.next() {
            let instrument_record: Instruments = result?;
            // println!("{:#?}", instrument_record);
            assert_eq!(
                instrument_record,
                Instruments {
                    instrument_token: 3813889,
                    exchange_token: 14898,
                    tradingsymbol: "CENTRALBK-BE".to_owned(),
                    name: Some("CENTRAL BANK OF INDIA".to_owned()),
                    last_price: 0.0,
                    expiry: None,
                    strike: 0.0,
                    tick_size: 0.05,
                    lot_size: 1,
                    instrument_type: InstrumentType::Equity,
                    segment: Segment::Nse,
                    exchange: Exchanges::Nse,
                }
            );
            // for result in deserialized {
            //     let instrument_record: Instruments = result?;
            //     println!("{:#?}", instrument_record);
            // }
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }
    #[test]
    fn test_instruments_all() -> Result<(), Box<dyn std::error::Error>> {
        let csvfile = Reader::from_path("../instruments_all.csv").unwrap();
        let mut deserialized = csvfile.into_deserialize::<Instruments>();
        if let Some(result) = deserialized.next() {
            let instrument_record: Instruments = result?;
            // println!("{:#?}", instrument_record);
            assert_eq!(
                instrument_record,
                Instruments {
                    instrument_token: 3813889,
                    exchange_token: 14898,
                    tradingsymbol: "CENTRALBK-BE".to_owned(),
                    name: Some("CENTRAL BANK OF INDIA".to_owned()),
                    last_price: 0.0,
                    expiry: None,
                    strike: 0.0,
                    tick_size: 0.05,
                    lot_size: 1,
                    instrument_type: InstrumentType::Equity,
                    segment: Segment::Nse,
                    exchange: Exchanges::Nse,
                }
            );
            // for result in deserialized {
            //     let instrument_record: Instruments = result?;
            //     println!("{:#?}", instrument_record);
            // }
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }
    #[test]
    fn test_instruments_actual() -> Result<(), Box<dyn std::error::Error>> {
        let csvfile = Reader::from_path("custom_mock_files/instruments.csv").unwrap();
        let mut deserialized = csvfile.into_deserialize::<Instruments>();
        if let Some(result) = deserialized.next() {
            let instrument_record: Instruments = result?;
            println!("{:#?}", instrument_record);
            assert_eq!(
                instrument_record,
                Instruments {
                    instrument_token: 547237382,
                    exchange_token: 2137646,
                    tradingsymbol: "EURINR22AUG79.75CE".to_owned(),
                    name: Some("EURINR".to_owned()),
                    last_price: 0.0,
                    expiry: Some(Expiry::NaiveDate(NaiveDate::from_ymd(2022, 8, 26))),
                    strike: 79.75,
                    tick_size: 0.0025,
                    lot_size: 1,
                    instrument_type: InstrumentType::CallOption,
                    segment: Segment::BcdOptions,
                    exchange: Exchanges::Bcd,
                }
            );
            // for result in deserialized {
            //     let instrument_record: Instruments = result?;
            //     println!("{:#?}", instrument_record);
            // }
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }
}
