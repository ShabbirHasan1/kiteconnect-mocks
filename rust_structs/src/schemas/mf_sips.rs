use super::{common::*, mf_sip_info::*};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfSips {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MfSipInfoData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    use chrono::NaiveDate;
    use std::collections::HashMap;

    #[test]
    fn test_mf_sips_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../mf_sips.json").unwrap();
        let deserialized: MfSips = simd_json::from_reader(jsonfile)?;
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
                    MfSipInfoData {
                        status: MfSipStatus::Active,
                        sip_reg_num: None,
                        pending_instalments: -1,
                        created: Some(NaiveDate::from_ymd(2021, 5, 5).and_hms(5, 56, 27)),
                        step_up: Meta::HashMap(step_up_1),
                        dividend_type: DividendType::Idcw,
                        instalment_amount: 500.0,
                        tag: Some(MfSipTags::Enum(MfSipTagsValue::CoinIosSip)),
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
                        fund_source: None,
                        completed_instalments: 0,
                        sip_id: "892741486820670".into(),
                    },
                    MfSipInfoData {
                        status: MfSipStatus::Active,
                        sip_reg_num: None,
                        pending_instalments: -1,
                        created: Some(NaiveDate::from_ymd(2021, 5, 25).and_hms(10, 55, 9)),
                        step_up: Meta::HashMap(step_up_2),
                        dividend_type: DividendType::Idcw,
                        instalment_amount: 1000.0,
                        tag: Some(MfSipTags::Enum(MfSipTagsValue::CoinIosSip)),
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
                        fund_source: None,
                        completed_instalments: 0,
                        sip_id: "109195857904698".into(),
                    },
                    MfSipInfoData {
                        status: MfSipStatus::Active,
                        sip_reg_num: Some("15158182".into(),),
                        pending_instalments: 9998,
                        created: Some(NaiveDate::from_ymd(2021, 5, 22).and_hms(10, 45, 29)),
                        step_up: Meta::HashMap(HashMap::new()),
                        dividend_type: DividendType::Idcw,
                        instalment_amount: 1000.0,
                        tag: Some(MfSipTags::Enum(MfSipTagsValue::CoinAndroidSip)),
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
                        fund_source: None,
                        completed_instalments: 1,
                        sip_id: "846479755969168".into(),
                    },
                    MfSipInfoData {
                        status: MfSipStatus::Active,
                        sip_reg_num: Some("16055666".into(),),
                        pending_instalments: 9998,
                        created: Some(NaiveDate::from_ymd(2021, 6, 18).and_hms(3, 56, 46)),
                        step_up: Meta::HashMap(HashMap::new()),
                        dividend_type: DividendType::Idcw,
                        instalment_amount: 1000.0,
                        tag: Some(MfSipTags::Enum(MfSipTagsValue::CoinAndroidSip)),
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
                        fund_source: None,
                        completed_instalments: 1,
                        sip_id: "749073272501476".into(),
                    },
                    MfSipInfoData {
                        status: MfSipStatus::Active,
                        sip_reg_num: None,
                        pending_instalments: -1,
                        created: Some(NaiveDate::from_ymd(2020, 11, 20).and_hms(1, 6, 11)),
                        step_up: Meta::HashMap(step_up_3),
                        dividend_type: DividendType::Growth,
                        instalment_amount: 7427.0,
                        tag: Some(MfSipTags::Enum(MfSipTagsValue::CoinAndroidSip)),
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
                        fund_source: None,
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
    fn test_mf_sips_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#.to_owned();
        let deserialized: MfSips = simd_json::from_str(raw_data.borrow_mut())?;
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
