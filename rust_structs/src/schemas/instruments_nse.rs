use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Instruments<'a> {
    instrument_token: u64,
    exchange_token: u64,
    #[serde(borrow)]
    tradingsymbol: &'a str,
    #[serde(borrow, deserialize_with = "csv::invalid_option")]
    name: Option<&'a str>,
    last_price: f64,
    #[serde(borrow)]
    expiry: Option<&'a str>,
    strike: f64,
    tick_size: f64,
    lot_size: u64,
    #[serde(borrow)]
    instrument_type: &'a str,
    #[serde(borrow)]
    segment: &'a str,
    #[serde(borrow)]
    exchange: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    // use chrono::NaiveDate;
    use csv::Reader;
    #[test]
    fn test_instruments_nse_nse() -> Result<(), Box<dyn std::error::Error>> {
        let mut csvfile = Reader::from_path("../instruments_nse.csv").unwrap();
        let mut raw_record = csv::ByteRecord::new();
        let headers = csvfile.byte_headers()?.clone();
        if csvfile.read_byte_record(&mut raw_record)? {
            let instrument_record: Instruments = raw_record.deserialize(Some(&headers))?;
            println!("{:#?}", instrument_record);
            assert_eq!(
                instrument_record,
                Instruments {
                    instrument_token: 3813889,
                    exchange_token: 14898,
                    tradingsymbol: "CENTRALBK-BE",
                    name: Some("CENTRAL BANK OF INDIA"),
                    last_price: 0.0,
                    expiry: None,
                    strike: 0.0,
                    tick_size: 0.05,
                    lot_size: 1,
                    instrument_type: "EQ",
                    segment: "NSE",
                    exchange: "NSE",
                }
            );
            while csvfile.read_byte_record(&mut raw_record)? {
                let instrument_record: Instruments = raw_record.deserialize(Some(&headers))?;
                println!("{:#?}", instrument_record);
            }
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }
    #[test]
    fn test_instruments_nse_all() -> Result<(), Box<dyn std::error::Error>> {
        let mut csvfile = Reader::from_path("../instruments_all.csv").unwrap();
        let mut raw_record = csv::ByteRecord::new();
        let headers = csvfile.byte_headers()?.clone();
        if csvfile.read_byte_record(&mut raw_record)? {
            let instrument_record: Instruments = raw_record.deserialize(Some(&headers))?;
            println!("{:#?}", instrument_record);
            assert_eq!(
                instrument_record,
                Instruments {
                    instrument_token: 3813889,
                    exchange_token: 14898,
                    tradingsymbol: "CENTRALBK-BE",
                    name: Some("CENTRAL BANK OF INDIA"),
                    last_price: 0.0,
                    expiry: None,
                    strike: 0.0,
                    tick_size: 0.05,
                    lot_size: 1,
                    instrument_type: "EQ",
                    segment: "NSE",
                    exchange: "NSE",
                }
            );
            while csvfile.read_byte_record(&mut raw_record)? {
                let instrument_record: Instruments = raw_record.deserialize(Some(&headers))?;
                println!("{:#?}", instrument_record);
            }
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }
    #[test]
    fn test_instruments_nse_actual() -> Result<(), Box<dyn std::error::Error>> {
        let mut csvfile = Reader::from_path("custom_mock_files/instruments.csv").unwrap();
        let mut raw_record = csv::ByteRecord::new();
        let headers = csvfile.byte_headers()?.clone();
        if csvfile.read_byte_record(&mut raw_record)? {
            let instrument_record: Instruments = raw_record.deserialize(Some(&headers))?;
            println!("{:#?}", instrument_record);
            assert_eq!(
                instrument_record,
                Instruments {
                    instrument_token: 547237382,
                    exchange_token: 2137646,
                    tradingsymbol: "EURINR22AUG79.75CE",
                    name: Some("EURINR"),
                    last_price: 0.0,
                    // expiry: Some(Expiry::NaiveDate(NaiveDate::from_ymd(2022, 8, 26))),
                    expiry: Some("2022-08-26"),
                    strike: 79.75,
                    tick_size: 0.0025,
                    lot_size: 1,
                    instrument_type: "CE",
                    segment: "BCD-OPT",
                    exchange: "BCD",
                }
            );
            while csvfile.read_byte_record(&mut raw_record)? {
                let instrument_record: Instruments = raw_record.deserialize(Some(&headers))?;
                println!("{:#?}", instrument_record);
            }
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }
}
