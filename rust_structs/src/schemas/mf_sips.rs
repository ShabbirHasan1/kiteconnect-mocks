use super::common::*;
use crate::utils::*;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfSips {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MfSipsData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfSipsData {
    pub status: MfSipStatus,
    pub sip_reg_num: Option<String>,
    pub pending_instalments: i64,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<NaiveDateTime>,
    pub step_up: Meta,
    pub dividend_type: DividendType,
    pub instalment_amount: f64,
    pub tag: String,
    pub instalments: i64,
    #[serde(
        with = "optional_naive_date_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_instalment: Option<NaiveDate>,
    pub transaction_type: TransactionType,
    pub trigger_price: f64,
    pub fund: String,
    pub tradingsymbol: String,
    pub frequency: SipFrequency,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_instalment: Option<NaiveDateTime>,
    pub instalment_day: u8,
    pub sip_type: Variety,
    pub completed_instalments: u64,
    pub sip_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::collections::HashMap;

    #[test]
    fn test_mf_sips_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../mf_sips.json").unwrap();
        let deserialized: MfSips = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        let mut step_up_1 = HashMap::<String, serde_json::Value>::new();
        step_up_1.insert("05-05".to_string(), serde_json::Number::from(10).into());
        let mut step_up_2 = HashMap::<String, serde_json::Value>::new();
        step_up_2.insert("25-05".to_string(), serde_json::Number::from(10).into());
        let mut step_up_3 = HashMap::<String, serde_json::Value>::new();
        step_up_3.insert("20-11".to_string(), serde_json::Number::from(30).into());
        assert_eq!(
            deserialized,
            MfSips {
                status: Status::Success,
                data: Some(vec![
                    MfSipsData {
                        status: MfSipStatus::Active,
                        sip_reg_num: None,
                        pending_instalments: -1,
                        created: Some(NaiveDate::from_ymd(2021, 5, 5).and_hms(5, 56, 27)),
                        step_up: Meta::HashMap(step_up_1),
                        dividend_type: DividendType::Idcw,
                        instalment_amount: 500.0,
                        tag: "coiniossip".into(),
                        instalments: -1,
                        next_instalment: Some(NaiveDate::from_ymd(2021, 5, 12)),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        fund: "Aditya Birla Sun Life Liquid Fund - Direct Plan".into(),
                        tradingsymbol: "INF209K01VD7".into(),
                        frequency: SipFrequency::Weekly,
                        last_instalment: Some(NaiveDate::from_ymd(2021, 5, 5).and_hms(5, 56, 27)),
                        instalment_day: 0,
                        sip_type: Variety::Sip,
                        completed_instalments: 0,
                        sip_id: "892741486820670".into(),
                    },
                    MfSipsData {
                        status: MfSipStatus::Active,
                        sip_reg_num: None,
                        pending_instalments: -1,
                        created: Some(NaiveDate::from_ymd(2021, 5, 25).and_hms(10, 55, 9)),
                        step_up: Meta::HashMap(step_up_2),
                        dividend_type: DividendType::Idcw,
                        instalment_amount: 1000.0,
                        tag: "coiniossip".into(),
                        instalments: -1,
                        next_instalment: Some(NaiveDate::from_ymd(2021, 6, 1)),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        fund: "HDFC Balanced Advantage Fund - Direct Plan".into(),
                        tradingsymbol: "INF179K01VY8".into(),
                        frequency: SipFrequency::Weekly,
                        last_instalment: Some(NaiveDate::from_ymd(2021, 5, 25).and_hms(10, 55, 9)),
                        instalment_day: 0,
                        sip_type: Variety::Sip,
                        completed_instalments: 0,
                        sip_id: "109195857904698".into(),
                    },
                    MfSipsData {
                        status: MfSipStatus::Active,
                        sip_reg_num: Some("15158182".into(),),
                        pending_instalments: 9998,
                        created: Some(NaiveDate::from_ymd(2021, 5, 22).and_hms(10, 45, 29)),
                        step_up: Meta::HashMap(HashMap::new()),
                        dividend_type: DividendType::Idcw,
                        instalment_amount: 1000.0,
                        tag: "coinandroidsip".into(),
                        instalments: 9999,
                        next_instalment: Some(NaiveDate::from_ymd(2021, 7, 12)),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        fund: "HDFC Balanced Advantage Fund - Direct Plan".into(),
                        tradingsymbol: "INF179K01VY8".into(),
                        frequency: SipFrequency::Monthly,
                        last_instalment: Some(NaiveDate::from_ymd(2021, 6, 10).and_hms(8, 37, 11)),
                        instalment_day: 10,
                        sip_type: Variety::AmcSip,
                        completed_instalments: 1,
                        sip_id: "846479755969168".into(),
                    },
                    MfSipsData {
                        status: MfSipStatus::Active,
                        sip_reg_num: Some("16055666".into(),),
                        pending_instalments: 9998,
                        created: Some(NaiveDate::from_ymd(2021, 6, 18).and_hms(3, 56, 46)),
                        step_up: Meta::HashMap(HashMap::new()),
                        dividend_type: DividendType::Idcw,
                        instalment_amount: 1000.0,
                        tag: "coinandroidsip".into(),
                        instalments: 9999,
                        next_instalment: Some(NaiveDate::from_ymd(2021, 7, 30)),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        fund: "HDFC Balanced Advantage Fund - Direct Plan".into(),
                        tradingsymbol: "INF179K01VY8".into(),
                        frequency: SipFrequency::Monthly,
                        last_instalment: Some(NaiveDate::from_ymd(2021, 6, 30).and_hms(8, 33, 7)),
                        instalment_day: 30,
                        sip_type: Variety::AmcSip,
                        completed_instalments: 1,
                        sip_id: "749073272501476".into(),
                    },
                    MfSipsData {
                        status: MfSipStatus::Active,
                        sip_reg_num: None,
                        pending_instalments: -1,
                        created: Some(NaiveDate::from_ymd(2020, 11, 20).and_hms(1, 6, 11)),
                        step_up: Meta::HashMap(step_up_3),
                        dividend_type: DividendType::Growth,
                        instalment_amount: 7427.0,
                        tag: "coinandroidsip".into(),
                        instalments: -1,
                        next_instalment: Some(NaiveDate::from_ymd(2021, 2, 19)),
                        transaction_type: TransactionType::Buy,
                        trigger_price: 0.0,
                        fund: "HDFC Hybrid Equity Fund - Direct Plan".into(),
                        tradingsymbol: "INF179K01XZ1".into(),
                        frequency: SipFrequency::Quarterly,
                        last_instalment: Some(NaiveDate::from_ymd(2020, 11, 20).and_hms(1, 6, 11)),
                        instalment_day: 0,
                        sip_type: Variety::Sip,
                        completed_instalments: 0,
                        sip_id: "576440634181776".into(),
                    },
                ],),
                ..MfSips::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_mf_sips_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: MfSips = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfSips {
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
