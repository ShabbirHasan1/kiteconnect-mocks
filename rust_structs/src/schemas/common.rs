use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaIceberg {
    pub iceberg: Iceberg,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Iceberg {
    pub leg: i64,
    pub legs: i64,
    pub leg_quantity: i64,
    pub total_quantity: i64,
    pub remaining_quantity: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Meta {
    MetaIceberg(MetaIceberg),
    HashMap(HashMap<String, serde_json::Value>),
    String(String),
}

impl Default for Meta {
    fn default() -> Self {
        Meta::HashMap(HashMap::new())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tags {
    Array([TagsValue; 1]),
    Vector(Vec<String>),
    Enum(TagsValue),
    String(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TagsValue {
    #[default]
    #[serde(rename = "icebergord")]
    IcebergOrder,
}

impl Default for Tags {
    fn default() -> Self {
        Tags::Vector(Vec::new())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusCheck {
    status: Status,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
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
    Individual,
}
impl Default for UserType {
    fn default() -> Self {
        UserType::Enum(UserTypeEnum::Individual)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Broker {
    Enum(BrokerName),
    String(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BrokerName {
    #[default]
    Zerodha,
}

impl Default for Broker {
    fn default() -> Self {
        Broker::Enum(BrokerName::Zerodha)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Exchanges {
    Nse,
    Nfo,
    Bfo,
    Cds,
    Bse,
    Mcx,
    Bcd,
    Mf,
}
impl Default for Exchanges {
    fn default() -> Self {
        Exchanges::Nse
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionalExchanges {
    Enum(Exchanges),
    String(String),
}

impl Default for OptionalExchanges {
    fn default() -> Self {
        OptionalExchanges::Enum(Exchanges::Nse)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Products {
    #[serde(rename = "CNC")]
    CashAndCarry, //Cash & Carry for equity
    #[serde(rename = "NRML")]
    Normal, //Normal for futures and options
    #[serde(rename = "MIS")]
    MarginIntradaySquareoff, //Margin Intraday Squareoff for futures and options
    #[serde(rename = "BO")]
    BracketOrder, // Bracket Order
    #[serde(rename = "CO")]
    CoverOrder, // Cover Order
}
impl Default for Products {
    fn default() -> Self {
        Products::CashAndCarry
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderTypes {
    #[serde(rename = "MARKET")]
    Market, //Market Order
    #[serde(rename = "LIMIT")]
    Limit, //Limit Order
    #[serde(rename = "SL")]
    StopLoss, //StopLoss Order
    #[serde(rename = "SL-M")]
    StopLossMarket, //StopLoss-Market Order
}
impl Default for OrderTypes {
    fn default() -> Self {
        OrderTypes::Market
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
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionType {
    Buy,
    Sell,
}

impl Default for TransactionType {
    fn default() -> Self {
        TransactionType::Buy
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Variety {
    Regular, // Regular order
    Amo,     // After Market Order
    Bo,      // Bracket Order
    Co,      // Cover Order
    Iceberg, // Iceberg Order
}

impl Default for Variety {
    fn default() -> Self {
        Variety::Regular
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Validity {
    Day, //Regular order
    Ioc, //Immediate or Cancel
    Ttl, //Order validity in minutes
}

impl Default for Validity {
    fn default() -> Self {
        Validity::Day
    }
}

impl Validity {
    pub fn is_not_ttl(&self) -> bool {
        match self {
            Validity::Day => true,
            Validity::Ioc => true,
            Validity::Ttl => false,
        }
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
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "PUT ORDER REQUEST RECEIVED")]
    PutOrderRequestReceived, // Order request has been received by the backend
    #[serde(rename = "PUT ORDER REQ RECEIVED")]
    PutOrderReqReceived, // Order request has been received by the backend
    #[serde(rename = "VALIDATION PENDING")]
    ValidationPending, // Order pending validation by the RMS (Risk Management System)
    #[serde(rename = "OPEN PENDING")]
    OpenPending, // Order is pending registration at the exchange
    #[serde(rename = "MODIFY VALIDATION PENDING")]
    ModifyValidationPending, // Order's modification values are pending validation by the RMS
    #[serde(rename = "MODIFY PENDING")]
    ModifyPending, // Order's modification values are pending registration at the exchange
    #[serde(rename = "TRIGGER PENDING")]
    TriggerPending, // Order's placed but the fill is pending based on a trigger price.
    #[serde(rename = "CANCEL PENDING")]
    CancelPending, // Order's cancellation request is pending registration at the exchange
    #[serde(rename = "AMO REQ RECEIVED")]
    AmoReqReceived, // Same as PUT ORDER REQUEST RECEIVED, but for AMOs
    #[serde(rename = "MODIFIED")]
    Modified,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::Enum(OrderStatusValue::Open)
    }
}

impl Default for OrderStatusValue {
    fn default() -> Self {
        OrderStatusValue::Open
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionalOrderMarginTypes {
    Enum(OrderMarginTypes),
    String(String),
}

impl Default for OptionalOrderMarginTypes {
    fn default() -> Self {
        OptionalOrderMarginTypes::Enum(OrderMarginTypes::Equity)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InstrumentType {
    #[serde(rename = "EQ")]
    Equity,
    #[serde(rename = "PE")]
    PutOption,
    #[serde(rename = "CE")]
    CallOption,
    #[serde(rename = "FUT")]
    Future,
}

impl Default for InstrumentType {
    fn default() -> Self {
        InstrumentType::Equity
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Segment {
    #[serde(rename = "BCD-OPT")]
    BcdOptions,
    #[serde(rename = "BCD-FUT")]
    BcdFutures,
    #[serde(rename = "BCD")]
    Bcd,
    #[serde(rename = "BSE")]
    Bse,
    #[serde(rename = "INDICES")]
    Indices,
    #[serde(rename = "CDS-OPT")]
    CdsOptions,
    #[serde(rename = "CDS-FUT")]
    CdsFutures,
    #[serde(rename = "MCX-FUT")]
    McxFutures,
    #[serde(rename = "MCX-OPT")]
    McxOptions,
    #[serde(rename = "NFO-OPT")]
    NfoOptions,
    #[serde(rename = "NFO-FUT")]
    NfoFutures,
    #[serde(rename = "NSE")]
    Nse,
}

impl Default for Segment {
    fn default() -> Self {
        Segment::Nse
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
