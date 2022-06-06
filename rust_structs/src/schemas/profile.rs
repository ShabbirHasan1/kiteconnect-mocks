use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProfileData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileData {
    pub user_id: String,
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
    pub meta: ProfileMeta,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    #[test]
    fn test_profile_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../profile.json").unwrap();
        let deserialized: Profile = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Profile {
                status: Status::Success,
                data: Some(ProfileData {
                    user_id: "AB1234".to_owned(),
                    user_type: UserType::Enum(UserTypeEnum::Individual),
                    email: "xxxyyy@gmail.com".to_owned(),
                    user_name: "AxAx Bxx".to_owned(),
                    user_shortname: "AxAx".to_owned(),
                    broker: Broker::Enum(BrokerName::Zerodha),
                    exchanges: vec![
                        Exchanges::Bfo,
                        Exchanges::Mcx,
                        Exchanges::Nse,
                        Exchanges::Cds,
                        Exchanges::Bse,
                        Exchanges::Bcd,
                        Exchanges::Mf,
                        Exchanges::Nfo,
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
                    avatar_url: None,
                    meta: ProfileMeta {
                        demat_consent: ProfileMetaEnum::Enum(ProfileMetaValueEnum::Physical),
                    },
                }),
                ..Profile::default()
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
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#.to_owned();
        let deserialized: Profile = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Profile {
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
