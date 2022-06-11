use super::common::*;
use crate::utils::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerateSession {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GenerateSessionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerateSessionData {
    pub user_type: UserType,
    pub email: String,
    pub user_name: String,
    pub user_shortname: String,
    pub broker: Broker,
    pub exchanges: Vec<Exchanges>,
    pub products: Vec<Products>,
    pub order_types: Vec<OrderTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub user_id: String,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enctoken: Option<String>,
    #[serde(
        with = "serde_with::rust::string_empty_as_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token: Option<String>,
    #[serde(
        with = "serde_with::rust::string_empty_as_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub silo: Option<String>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub login_time: Option<NaiveDateTime>,
    pub meta: ProfileMeta,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::borrow::BorrowMut;
    #[test]
    fn test_profile_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../generate_session.json").unwrap();
        let deserialized: GenerateSession = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            GenerateSession {
                status: Status::Success,
                data: Some(GenerateSessionData {
                    user_type: UserType::Enum(UserTypeEnum::Individual),
                    email: "XXXXXX".to_owned(),
                    user_name: "Kite Connect".to_owned(),
                    user_shortname: "Connect".to_owned(),
                    broker: Broker::Enum(BrokerName::Zerodha),
                    exchanges: vec![
                        Exchanges::Nse,
                        Exchanges::Nfo,
                        Exchanges::Bfo,
                        Exchanges::Cds,
                        Exchanges::Bse,
                        Exchanges::Mcx,
                        Exchanges::Bcd,
                        Exchanges::Mf,
                    ],
                    products: vec![
                        Products::CashAndCarry,
                        Products::Normal,
                        Products::MarginIntradaySquareoff,
                        Products::BracketOrder,
                        Products::CoverOrder,
                    ],
                    order_types: vec![
                        OrderTypes::Market,
                        OrderTypes::Limit,
                        OrderTypes::StopLoss,
                        OrderTypes::StopLossMarket,
                    ],
                    avatar_url: Some("abc".to_owned()),
                    user_id: "XX0000".to_owned(),
                    api_key: "XXXXXX".to_owned(),
                    access_token: Some("XXXXXX".to_owned()),
                    public_token: Some("XXXXXXXX".to_owned()),
                    enctoken: Some("XXXXXX".to_owned()),
                    refresh_token: None,
                    silo: None,
                    login_time: Some(NaiveDate::from_ymd(2021, 1, 1).and_hms(16, 15, 14)),
                    meta: ProfileMeta {
                        demat_consent: ProfileMetaEnum::Enum(ProfileMetaValueEnum::Physical),
                    },
                }),
                ..GenerateSession::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_profile_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#
                .to_owned();
        let deserialized: GenerateSession = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            GenerateSession {
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
