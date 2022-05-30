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
        with = "optional_naive_date_time_with_milli_or_micro_from_str",
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
    use std::collections::HashMap;

    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_mf_sip_info_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../mf_sip_info.json").unwrap();
        let deserialized: MfSipInfo = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfSipInfo {
                status: Status::Success,
                data: Some(MfSipInfoData {
                    status: MfSipStatus::Active,
                    sip_reg_num: Some("15158182".into()),
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
                    last_instalment: Some(
                        NaiveDate::from_ymd(2021, 6, 10).and_hms_micro(8, 37, 11, 273142)
                    ),
                    instalment_day: 10,
                    sip_type: Variety::AmcSip,
                    completed_instalments: 1,
                    sip_id: "846479755969168".into(),
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
    fn test_mf_sip_info_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: MfSipInfo = serde_json::from_str(&raw_data)?;
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
