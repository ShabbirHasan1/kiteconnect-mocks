use super::common::*;
use crate::utils::*;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfSipInfo {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MfSipInfoData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfSipInfoData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<MfSipTags>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_source: Option<String>,
    pub completed_instalments: u64,
    pub sip_id: String,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use std::borrow::BorrowMut;
    use chrono::NaiveDate;
    #[test]
    fn test_mf_sip_info_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../mf_sip_info.json").unwrap();
        let deserialized: MfSipInfo = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        let mut step_up_val = HashMap::<String, serde_json::Value>::new();
        step_up_val.insert("15-02".to_string(), serde_json::Number::from(10).into());
        assert_eq!(
            deserialized,
            MfSipInfo {
                status: Status::Success,
                data: Some(MfSipInfoData {
                    status: MfSipStatus::Active,
                    sip_reg_num: None, // Some("15158182".into()),
                    pending_instalments: -1,
                    created: Some(NaiveDate::from_ymd(2022, 2, 15).and_hms(13, 39, 44)),
                    step_up: Meta::HashMap(step_up_val),
                    dividend_type: DividendType::Growth,
                    instalment_amount: 5000.00,
                    tag: Some(MfSipTags::Enum(MfSipTagsValue::CoinIosSip)),
                    instalments: -1,
                    next_instalment: Some(NaiveDate::from_ymd(2022, 2, 22)),
                    transaction_type: TransactionType::Buy,
                    trigger_price: 0.0,
                    fund: "Aditya Birla Sun Life Overnight Fund - Direct Plan".into(),
                    tradingsymbol: "INF209KB1ZH2".into(),
                    frequency: SipFrequency::Weekly,
                    last_instalment: Some(NaiveDate::from_ymd(2022, 2, 15).and_hms(13, 39, 44)),
                    instalment_day: 0,
                    sip_type: Variety::Sip,
                    fund_source: Some("pool".into()),
                    completed_instalments: 0,
                    sip_id: "181635213661372".into(),
                },),
                ..MfSipInfo::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_mf_sip_info_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#.to_owned();
        let deserialized: MfSipInfo = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfSipInfo {
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
