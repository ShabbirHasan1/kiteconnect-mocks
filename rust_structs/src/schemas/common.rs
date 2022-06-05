use serde::{Deserialize, Serialize};
use speedy::{Endianness, Readable, Writable};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
pub struct MetaIceberg {
    pub iceberg: Iceberg,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
pub struct Iceberg {
    pub leg: i64,
    pub legs: i64,
    pub leg_quantity: i64,
    pub total_quantity: i64,
    pub remaining_quantity: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum Meta {
    MetaIceberg(MetaIceberg),
    HashMap(HashMap<String, String>),
    String(String),
}

impl Default for Meta {
    fn default() -> Self {
        Self::HashMap(HashMap::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum Tags {
    Array([TagsValue; 1]),
    Vector(Vec<String>),
    Enum(TagsValue),
    String(String),
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
pub enum TagsValue {
    #[default]
    #[serde(rename = "icebergord")]
    IcebergOrder,
}

impl Default for Tags {
    fn default() -> Self {
        Self::Vector(Vec::new())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
pub struct StatusCheck {
    status: Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error,
    Failed,
}

impl Default for Status {
    fn default() -> Self {
        Self::Success
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
pub enum Exception {
    TokenException,
    UserException,
    OrderException,
    InputException,
    NetworkException,
    DataException,
    GeneralException,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum UserType {
    Enum(UserTypeEnum),
    String(String),
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum UserTypeEnum {
    #[default]
    Individual,
}
impl Default for UserType {
    fn default() -> Self {
        Self::Enum(UserTypeEnum::Individual)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum Broker {
    Enum(BrokerName),
    String(String),
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "UPPERCASE")]
pub enum BrokerName {
    #[default]
    Zerodha,
}

impl Default for Broker {
    fn default() -> Self {
        Self::Enum(BrokerName::Zerodha)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
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
        Self::Nse
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum OptionalExchanges {
    Enum(Exchanges),
    String(String),
}

impl Default for OptionalExchanges {
    fn default() -> Self {
        Self::Enum(Exchanges::Nse)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
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
        Self::CashAndCarry
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
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
        Self::Market
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
pub struct ProfileMeta {
    pub demat_consent: ProfileMetaEnum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum ProfileMetaEnum {
    Enum(ProfileMetaValueEnum),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum ProfileMetaValueEnum {
    Empty,
    Consent,
    Physical,
}

impl Default for ProfileMetaEnum {
    fn default() -> Self {
        Self::Enum(ProfileMetaValueEnum::Empty)
    }
}

impl Default for ProfileMetaValueEnum {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionType {
    Buy,
    Sell,
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Buy
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
pub enum Variety {
    #[serde(rename = "regular")]
    Regular, // Regular order
    #[serde(rename = "amo")]
    Amo, // After Market Order
    #[serde(rename = "bo")]
    Bo, // Bracket Order
    #[serde(rename = "co")]
    Co, // Cover Order
    #[serde(rename = "iceberg")]
    Iceberg, // Iceberg Order
    #[serde(rename = "sip")]
    Sip,
    #[serde(rename = "amc_sip")]
    AmcSip,
}

impl Default for Variety {
    fn default() -> Self {
        Self::Regular
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "UPPERCASE")]
pub enum Validity {
    Day, //Regular order
    Ioc, //Immediate or Cancel
    Ttl, //Order validity in minutes
}

impl Default for Validity {
    fn default() -> Self {
        Self::Day
    }
}

impl Validity {
    #[must_use]
    pub const fn is_not_ttl(&self) -> bool {
        match self {
            Self::Day | Self::Ioc => true,
            Self::Ttl => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum OrderStatus {
    Enum(OrderStatusValue),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
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
        Self::Enum(OrderStatusValue::Open)
    }
}

impl Default for OrderStatusValue {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum OrderMarginTypes {
    Equity,
    Commodity,
}

impl Default for OrderMarginTypes {
    fn default() -> Self {
        Self::Equity
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum OptionalOrderMarginTypes {
    Enum(OrderMarginTypes),
    String(String),
}

impl Default for OptionalOrderMarginTypes {
    fn default() -> Self {
        Self::Enum(OrderMarginTypes::Equity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
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
        Self::Equity
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
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
        Self::Nse
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum DividendType {
    Growth,
    Payout,
    Idcw,
}

impl Default for DividendType {
    fn default() -> Self {
        Self::Growth
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "UPPERCASE")]
pub enum MfSipStatus {
    Active,
    Paused,
    Cancelled,
}

impl Default for MfSipStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum GttOrderStatus {
    Active,
    Triggered,
    Disabled,
    Expired,
    Cancelled,
    Rejected,
    Deleted,
}

impl Default for GttOrderStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
pub enum GttOrderType {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "two-leg")]
    TwoLeg,
}

impl Default for GttOrderType {
    fn default() -> Self {
        Self::Single
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum MfPurchaseType {
    Enum(MfPurchaseTypeValue),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "UPPERCASE")]
pub enum MfPurchaseTypeValue {
    Fresh,
    Additional,
}

impl Default for MfPurchaseType {
    fn default() -> Self {
        Self::Enum(MfPurchaseTypeValue::Fresh)
    }
}

impl Default for MfPurchaseTypeValue {
    fn default() -> Self {
        Self::Fresh
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum SipFrequency {
    Daily,
    Weekly,
    BiWeekly,
    Fortnightly,
    Monthly,
    Quarterly,
    HalfYearly,
    Yearly,
    Annualy,
}

impl Default for SipFrequency {
    fn default() -> Self {
        Self::Daily
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
pub enum SettlementType {
    #[serde(rename = "T1")]
    Tone,
    #[serde(rename = "T2")]
    Ttwo,
    #[serde(rename = "T3")]
    Tthree,
    #[serde(rename = "T4")]
    Tfour,
    #[serde(rename = "T5")]
    Tfive,
    #[serde(rename = "T6")]
    Tsix,
    #[serde(rename = "T7")]
    Tseven,
}

impl Default for SettlementType {
    fn default() -> Self {
        Self::Tone
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum MfPlan {
    Direct,
    Regular,
}

impl Default for MfPlan {
    fn default() -> Self {
        Self::Direct
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum MfScheme {
    Balanced,
    Debt,
    Equity,
    Fof,
    Liquid,
}

impl Default for MfScheme {
    fn default() -> Self {
        Self::Balanced
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(untagged)]
pub enum MfSipTags {
    Enum(MfSipTagsValue),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Readable, Writable)]
#[serde(rename_all = "lowercase")]
pub enum MfSipTagsValue {
    CoinAndroidSip,
    CoinIosSip,
    CoinWebSip,
}

impl Default for MfSipTags {
    fn default() -> Self {
        Self::Enum(MfSipTagsValue::CoinWebSip)
    }
}

impl Default for MfSipTagsValue {
    fn default() -> Self {
        Self::CoinWebSip
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status_success_json() -> serde_json::Result<()> {
        let raw_data = r#"{"status":"success"}"#;
        let deserialized: StatusCheck = serde_json::from_str(raw_data)?;
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
        let deserialized: StatusCheck = serde_json::from_str(raw_data)?;
        assert_eq!(
            deserialized,
            StatusCheck {
                status: Status::Error
            }
        );
        Ok(())
    }
}
