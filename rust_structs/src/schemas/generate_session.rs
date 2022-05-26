use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateSession {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GenerateSessionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateSessionData {
    pub user_type: UserType,
    pub email: String,
    pub user_name: String,
    pub user_shortname: String,
    pub broker: Broker,
    pub exchanges: Vec<Exchanges>,
    pub products: Vec<Products>,
    pub order_types: Vec<OrderTypes>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub user_id: String,
    pub api_key: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub public_token: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub enctoken: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub silo: Option<String>,
    pub login_time: String,
    pub meta: ProfileMeta,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_profile_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_user_from_file("../generate_session.json").unwrap();
        let deserialized: GenerateSession = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            GenerateSession {
                status: Status::Success,
                data: Some(GenerateSessionData {
                    user_type: UserType::Enum(UserTypeEnum::INDIVIDUAL),
                    email: "XXXXXX".to_owned(),
                    user_name: "Kite Connect".to_owned(),
                    user_shortname: "Connect".to_owned(),
                    broker: Broker::Enum(BrokerName::ZERODHA),
                    exchanges: vec![
                        Exchanges::NSE,
                        Exchanges::NFO,
                        Exchanges::BFO,
                        Exchanges::CDS,
                        Exchanges::BSE,
                        Exchanges::MCX,
                        Exchanges::BCD,
                        Exchanges::MF
                    ],
                    products: vec![
                        Products::CNC,
                        Products::NRML,
                        Products::MIS,
                        Products::BO,
                        Products::CO,
                    ],
                    order_types: vec![
                        OrderTypes::MARKET,
                        OrderTypes::LIMIT,
                        OrderTypes::SL,
                        OrderTypes::SLM,
                    ],
                    avatar_url: Some("abc".to_owned()),
                    user_id: "XX0000".to_owned(),
                    api_key: "XXXXXX".to_owned(),
                    access_token: Some("XXXXXX".to_owned()),
                    public_token: Some("XXXXXXXX".to_owned()),
                    enctoken: Some("XXXXXX".to_owned()),
                    refresh_token: Some("".to_owned()),
                    silo: Some("".to_owned()),
                    login_time: "2021-01-01 16:15:14".to_owned(),
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
    fn test_profile_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: GenerateSession = serde_json::from_str(&raw_data)?;
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
