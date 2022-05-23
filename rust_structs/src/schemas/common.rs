use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusCheck {
    status: Status,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

impl Default for Status {
    fn default() -> Self {
        Status::Success
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Exception {
    TokenException,
    UserException,
    OrderException,
    InputException,
    NetworkException,
    DataException,
    GeneralException,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserType {
    Enum(UserTypeEnum),
    String(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserTypeEnum {
    #[default]
    INDIVIDUAL,
}
impl Default for UserType {
    fn default() -> Self {
        UserType::Enum(UserTypeEnum::INDIVIDUAL)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Broker {
    Enum(BrokerName),
    String(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BrokerName {
    #[default]
    ZERODHA,
}

impl Default for Broker {
    fn default() -> Self {
        Broker::Enum(BrokerName::ZERODHA)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Exchanges {
    NSE,
    NFO,
    BFO,
    CDS,
    BSE,
    MCX,
    BCD,
    MF,
}
impl Default for Exchanges {
    fn default() -> Self {
        Exchanges::NSE
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Products {
    CNC,
    NRML,
    MIS,
    BO,
    CO,
}
impl Default for Products {
    fn default() -> Self {
        Products::CNC
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderTypes {
    MARKET,
    LIMIT,
    SL,
    #[serde(rename = "SL-M")]
    SLM,
}
impl Default for OrderTypes {
    fn default() -> Self {
        OrderTypes::MARKET
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileMeta {
    pub demat_consent: ProfileMetaEnum,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileMetaEnum {
    Enum(ProfileMetaValueEnum),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileMetaValueEnum {
    Empty,
    Consent,
    Physical,
}

impl Default for ProfileMetaEnum {
    fn default() -> Self {
        ProfileMetaEnum::Enum(ProfileMetaValueEnum::Empty)
    }
}

impl Default for ProfileMetaValueEnum {
    fn default() -> Self {
        ProfileMetaValueEnum::Empty
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    BUY,
    SELL,
}

impl Default for TransactionType {
    fn default() -> Self {
        TransactionType::BUY
    }
}

#[test]
fn test_status_success_json() -> serde_json::Result<()> {
    let raw_data = r#"{"status":"success"}"#;
    let deserialized: StatusCheck = serde_json::from_str(&raw_data)?;
    assert_eq!(
        deserialized,
        StatusCheck {
            status: Status::Success
        }
    );
    Ok(())
}

#[test]
fn test_status_error_json() -> serde_json::Result<()> {
    let raw_data = r#"{"status":"error"}"#;
    let deserialized: StatusCheck = serde_json::from_str(&raw_data)?;
    assert_eq!(
        deserialized,
        StatusCheck {
            status: Status::Error
        }
    );
    Ok(())
}
