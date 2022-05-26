use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Meta {
    HashMap(HashMap<String, serde_json::Value>),
    String(String),
}

impl Default for Meta {
    fn default() -> Self {
        Meta::HashMap(HashMap::new())
    }
}

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
    CNC,  //Cash & Carry for equity
    NRML, //Normal for futures and options
    MIS,  //Margin Intraday Squareoff for futures and options
    BO,   // Bracket Order
    CO,   // Cover Order
}
impl Default for Products {
    fn default() -> Self {
        Products::CNC
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderTypes {
    MARKET, //Market order
    LIMIT,  //Limit order
    SL,     //Stoploss order
    #[serde(rename = "SL-M")]
    SLM, //Stoploss-market order
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Variety {
    Regular, //Regular order
    Amo,     //After Market Order
    Co,      //Cover Order
    Iceberg, //Iceberg Order
}

impl Default for Variety {
    fn default() -> Self {
        Variety::Regular
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Validity {
    DAY, //Regular order
    IOC, //Immediate or Cancel
    TTL, //Order validity in minutes
}

impl Default for Validity {
    fn default() -> Self {
        Validity::DAY
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderStatus {
    Enum(OrderStatusValue),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatusValue {
    COMPLETE,
    REJECTED,
    CANCELLED,
    OPEN,
    #[serde(rename = "PUT ORDER REQUEST RECEIVED")]
    PUTORDERREQUESTRECEIVED, // Order request has been received by the backend
    #[serde(rename = "PUT ORDER REQ RECEIVED")]
    PUTORDERREQRECEIVED, // Order request has been received by the backend
    #[serde(rename = "VALIDATION PENDING")]
    VALIDATIONPENDING, // Order pending validation by the RMS (Risk Management System)
    #[serde(rename = "OPEN PENDING")]
    OPENPENDING, // Order is pending registration at the exchange
    #[serde(rename = "MODIFY VALIDATION PENDING")]
    MODIFYVALIDATIONPENDING, // Order's modification values are pending validation by the RMS
    #[serde(rename = "MODIFY PENDING")]
    MODIFYPENDING, // Order's modification values are pending registration at the exchange
    #[serde(rename = "TRIGGER PENDING")]
    TRIGGERPENDING, // Order's placed but the fill is pending based on a trigger price.
    #[serde(rename = "CANCEL PENDING")]
    CANCELPENDING, // Order's cancellation request is pending registration at the exchange
    #[serde(rename = "AMO REQ RECEIVED")]
    AMOREQRECEIVED, // Same as PUT ORDER REQUEST RECEIVED, but for AMOs
    MODIFIED,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::Enum(OrderStatusValue::OPEN)
    }
}

impl Default for OrderStatusValue {
    fn default() -> Self {
        OrderStatusValue::OPEN
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderMarginTypes {
    Equity,
    Commodity,
}

impl Default for OrderMarginTypes {
    fn default() -> Self {
        OrderMarginTypes::Equity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
