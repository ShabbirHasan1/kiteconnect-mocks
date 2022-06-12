use super::common::*;
use super::ticker::*;
use crate::utils::*;
use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ltp {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
}

impl From<LtpBin> for Ltp {
    fn from(ltp_bin: LtpBin) -> Self {
        let instrument_token = i32::from_be_bytes(ltp_bin.instrument_token.try_into().unwrap());
        let segment = segment(instrument_token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(ltp_bin.last_price.try_into().unwrap()),
        );
        Self {
            tradable: tradable(segment),
            mode: TickerStreamingMode::Ltp,
            instrument_token,
            last_price,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ltpc {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
    pub close_price: f64,
}

impl From<LtpcBin> for Ltpc {
    fn from(ltpc_bin: LtpcBin) -> Self {
        let instrument_token = i32::from_be_bytes(ltpc_bin.instrument_token.try_into().unwrap());
        let segment = segment(instrument_token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(ltpc_bin.last_price.try_into().unwrap()),
        );
        let close_price = convert_price(
            segment,
            i32::from_be_bytes(ltpc_bin.close_price.try_into().unwrap()),
        );
        Self {
            tradable: tradable(segment),
            mode: TickerStreamingMode::Ltpc,
            instrument_token,
            last_price,
            close_price,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteIndex {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
    pub ohlc: TickerOhlc,
    pub change: f64,
    pub price_change: PriceChange,
}

impl From<QuoteIndexBin> for QuoteIndex {
    fn from(quote_index_bin: QuoteIndexBin) -> Self {
        let instrument_token =
            i32::from_be_bytes(quote_index_bin.instrument_token.try_into().unwrap());
        let segment = segment(instrument_token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(quote_index_bin.last_price.try_into().unwrap()),
        );
        let ohlc = TickerOhlc {
            open: convert_price(
                segment,
                i32::from_be_bytes(quote_index_bin.ohlc.open.try_into().unwrap()),
            ),
            high: convert_price(
                segment,
                i32::from_be_bytes(quote_index_bin.ohlc.high.try_into().unwrap()),
            ),
            low: convert_price(
                segment,
                i32::from_be_bytes(quote_index_bin.ohlc.low.try_into().unwrap()),
            ),
            close: convert_price(
                segment,
                i32::from_be_bytes(quote_index_bin.ohlc.close.try_into().unwrap()),
            ),
        };
        let change = i32::from_be_bytes(quote_index_bin.change.try_into().unwrap()) as f64;
        let price_change = calculate_change(last_price, ohlc.open, ohlc.close);
        Self {
            tradable: tradable(segment),
            mode: TickerStreamingMode::Quote,
            instrument_token,
            last_price,
            ohlc,
            change,
            price_change,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullIndex {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
    pub ohlc: TickerOhlc,
    pub change: f64,
    pub price_change: PriceChange,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub exchange_timestamp: DateTime<FixedOffset>,
}

impl From<FullIndexBin> for FullIndex {
    fn from(full_index_bin: FullIndexBin) -> Self {
        let instrument_token =
            i32::from_be_bytes(full_index_bin.instrument_token.try_into().unwrap());
        let segment = segment(instrument_token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(full_index_bin.last_price.try_into().unwrap()),
        );
        let ohlc = TickerOhlc {
            open: convert_price(
                segment,
                i32::from_be_bytes(full_index_bin.ohlc.open.try_into().unwrap()),
            ),
            high: convert_price(
                segment,
                i32::from_be_bytes(full_index_bin.ohlc.high.try_into().unwrap()),
            ),
            low: convert_price(
                segment,
                i32::from_be_bytes(full_index_bin.ohlc.low.try_into().unwrap()),
            ),
            close: convert_price(
                segment,
                i32::from_be_bytes(full_index_bin.ohlc.close.try_into().unwrap()),
            ),
        };
        let change = i32::from_be_bytes(full_index_bin.change.try_into().unwrap()) as f64;
        let price_change = calculate_change(last_price, ohlc.open, ohlc.close);
        let exchange_timestamp =
            i32::from_be_bytes(full_index_bin.exchange_timestamp.try_into().unwrap()) as i64;
        Self {
            tradable: tradable(segment),
            mode: TickerStreamingMode::Full,
            instrument_token,
            last_price,
            ohlc,
            change,
            price_change,
            exchange_timestamp: FixedOffset::east(19800).timestamp(exchange_timestamp, 0),
        }
    }
}

impl Default for FullIndex {
    fn default() -> Self {
        Self {
            tradable: Default::default(),
            mode: Default::default(),
            instrument_token: Default::default(),
            last_price: Default::default(),
            ohlc: Default::default(),
            change: Default::default(),
            price_change: Default::default(),
            exchange_timestamp: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TickerOhlc {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
    pub last_traded_quantity: i32,
    pub average_traded_price: f64,
    pub volume_traded: i32,
    pub total_buy_quantity: i32,
    pub total_sell_quantity: i32,
    pub ohlc: TickerOhlc,
    pub price_change: PriceChange,
}

impl From<QuoteBin> for Quote {
    fn from(quote_bin: QuoteBin) -> Self {
        let instrument_token = i32::from_be_bytes(quote_bin.instrument_token.try_into().unwrap());
        let segment = segment(instrument_token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(quote_bin.last_price.try_into().unwrap()),
        );
        let last_traded_quantity =
            i32::from_be_bytes(quote_bin.last_traded_quantity.try_into().unwrap());
        let average_traded_price = convert_price(
            segment,
            i32::from_be_bytes(quote_bin.average_traded_price.try_into().unwrap()),
        );
        let volume_traded = i32::from_be_bytes(quote_bin.volume_traded.try_into().unwrap());
        let total_buy_quantity =
            i32::from_be_bytes(quote_bin.total_buy_quantity.try_into().unwrap());
        let total_sell_quantity =
            i32::from_be_bytes(quote_bin.total_sell_quantity.try_into().unwrap());
        let ohlc = TickerOhlc {
            open: convert_price(
                segment,
                i32::from_be_bytes(quote_bin.ohlc.open.try_into().unwrap()),
            ),
            high: convert_price(
                segment,
                i32::from_be_bytes(quote_bin.ohlc.high.try_into().unwrap()),
            ),
            low: convert_price(
                segment,
                i32::from_be_bytes(quote_bin.ohlc.low.try_into().unwrap()),
            ),
            close: convert_price(
                segment,
                i32::from_be_bytes(quote_bin.ohlc.close.try_into().unwrap()),
            ),
        };
        let price_change = calculate_change(last_price, ohlc.open, ohlc.close);
        Self {
            tradable: tradable(segment),
            mode: TickerStreamingMode::Quote,
            instrument_token,
            last_price,
            last_traded_quantity,
            average_traded_price,
            volume_traded,
            total_buy_quantity,
            total_sell_quantity,
            ohlc,
            price_change,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Full {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
    pub last_traded_quantity: i32,
    pub average_traded_price: f64,
    pub volume_traded: i32,
    pub total_buy_quantity: i32,
    pub total_sell_quantity: i32,
    pub ohlc: TickerOhlc,
    pub price_change: PriceChange,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub last_trade_time: DateTime<FixedOffset>,
    pub oi: i32,
    pub oi_day_low: i32,
    pub oi_day_high: i32,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub exchange_timestamp: DateTime<FixedOffset>,
    pub depth: MarketDepth,
}

impl From<FullBin> for Full {
    fn from(full_bin: FullBin) -> Self {
        let instrument_token = i32::from_be_bytes(full_bin.instrument_token.try_into().unwrap());
        let segment = segment(instrument_token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(full_bin.last_price.try_into().unwrap()),
        );
        let last_traded_quantity =
            i32::from_be_bytes(full_bin.last_traded_quantity.try_into().unwrap());
        let average_traded_price = convert_price(
            segment,
            i32::from_be_bytes(full_bin.average_traded_price.try_into().unwrap()),
        );
        let volume_traded = i32::from_be_bytes(full_bin.volume_traded.try_into().unwrap());
        let total_buy_quantity =
            i32::from_be_bytes(full_bin.total_buy_quantity.try_into().unwrap());
        let total_sell_quantity =
            i32::from_be_bytes(full_bin.total_sell_quantity.try_into().unwrap());
        let ohlc = TickerOhlc {
            open: convert_price(
                segment,
                i32::from_be_bytes(full_bin.ohlc.open.try_into().unwrap()),
            ),
            high: convert_price(
                segment,
                i32::from_be_bytes(full_bin.ohlc.high.try_into().unwrap()),
            ),
            low: convert_price(
                segment,
                i32::from_be_bytes(full_bin.ohlc.low.try_into().unwrap()),
            ),
            close: convert_price(
                segment,
                i32::from_be_bytes(full_bin.ohlc.close.try_into().unwrap()),
            ),
        };
        let price_change = calculate_change(last_price, ohlc.open, ohlc.close);
        let last_trade_time =
            i32::from_be_bytes(full_bin.last_trade_time.try_into().unwrap()) as i64;
        let oi = i32::from_be_bytes(full_bin.oi.try_into().unwrap());
        let oi_day_low = i32::from_be_bytes(full_bin.oi_day_low.try_into().unwrap());
        let oi_day_high = i32::from_be_bytes(full_bin.oi_day_high.try_into().unwrap());
        let exchange_timestamp =
            i32::from_be_bytes(full_bin.exchange_timestamp.try_into().unwrap()) as i64;
        let depth = MarketDepth {
            buy: full_bin
                .depth
                .buy
                .into_iter()
                .map(|buy| {
                    let price =
                        convert_price(segment, i32::from_be_bytes(buy.price.try_into().unwrap()));
                    let quantity = i32::from_be_bytes(buy.quantity.try_into().unwrap());
                    let orders = i16::from_be_bytes(buy.orders.try_into().unwrap()) as i32;
                    MarketDepthData {
                        price,
                        quantity,
                        orders,
                    }
                })
                .collect(),
            sell: full_bin
                .depth
                .sell
                .into_iter()
                .map(|sell| {
                    let price =
                        convert_price(segment, i32::from_be_bytes(sell.price.try_into().unwrap()));
                    let quantity = i32::from_be_bytes(sell.quantity.try_into().unwrap());
                    let orders = i16::from_be_bytes(sell.orders.try_into().unwrap()) as i32;
                    MarketDepthData {
                        price,
                        quantity,
                        orders,
                    }
                })
                .collect(),
        };
        Self {
            tradable: tradable(segment),
            mode: TickerStreamingMode::Full,
            instrument_token,
            last_price,
            last_traded_quantity,
            average_traded_price,
            volume_traded,
            total_buy_quantity,
            total_sell_quantity,
            ohlc,
            price_change,
            last_trade_time: FixedOffset::east(19800).timestamp(last_trade_time, 0),
            oi,
            oi_day_low,
            oi_day_high,
            exchange_timestamp: FixedOffset::east(19800).timestamp(exchange_timestamp, 0),
            depth,
        }
    }
}

impl Default for Full {
    fn default() -> Self {
        Self {
            tradable: Default::default(),
            mode: Default::default(),
            instrument_token: Default::default(),
            last_price: Default::default(),
            last_traded_quantity: Default::default(),
            average_traded_price: Default::default(),
            volume_traded: Default::default(),
            total_buy_quantity: Default::default(),
            total_sell_quantity: Default::default(),
            ohlc: Default::default(),
            price_change: Default::default(),
            last_trade_time: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            oi: Default::default(),
            oi_day_low: Default::default(),
            oi_day_high: Default::default(),
            exchange_timestamp: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            depth: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompactFull {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
    pub last_traded_quantity: i32,
    pub average_traded_price: f64,
    pub volume_traded: i32,
    pub total_buy_quantity: i32,
    pub total_sell_quantity: i32,
    pub ohlc: TickerOhlc,
    pub price_change: PriceChange,
    pub depth: MarketDepth,
}

impl From<CompactFullBin> for CompactFull {
    fn from(compact_full: CompactFullBin) -> Self {
        let instrument_token =
            i32::from_be_bytes(compact_full.instrument_token.try_into().unwrap());
        let segment = segment(instrument_token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(compact_full.last_price.try_into().unwrap()),
        );
        let last_traded_quantity =
            i32::from_be_bytes(compact_full.last_traded_quantity.try_into().unwrap());
        let average_traded_price = convert_price(
            segment,
            i32::from_be_bytes(compact_full.average_traded_price.try_into().unwrap()),
        );
        let volume_traded = i32::from_be_bytes(compact_full.volume_traded.try_into().unwrap());
        let total_buy_quantity =
            i32::from_be_bytes(compact_full.total_buy_quantity.try_into().unwrap());
        let total_sell_quantity =
            i32::from_be_bytes(compact_full.total_sell_quantity.try_into().unwrap());
        let ohlc = TickerOhlc {
            open: convert_price(
                segment,
                i32::from_be_bytes(compact_full.ohlc.open.try_into().unwrap()),
            ),
            high: convert_price(
                segment,
                i32::from_be_bytes(compact_full.ohlc.high.try_into().unwrap()),
            ),
            low: convert_price(
                segment,
                i32::from_be_bytes(compact_full.ohlc.low.try_into().unwrap()),
            ),
            close: convert_price(
                segment,
                i32::from_be_bytes(compact_full.ohlc.close.try_into().unwrap()),
            ),
        };
        let price_change = calculate_change(last_price, ohlc.open, ohlc.close);
        let depth = MarketDepth {
            buy: compact_full
                .depth
                .buy
                .into_iter()
                .map(|buy| {
                    let price =
                        convert_price(segment, i32::from_be_bytes(buy.price.try_into().unwrap()));
                    let quantity = i32::from_be_bytes(buy.quantity.try_into().unwrap());
                    let orders = i16::from_be_bytes(buy.orders.try_into().unwrap()) as i32;
                    MarketDepthData {
                        price,
                        quantity,
                        orders,
                    }
                })
                .collect(),
            sell: compact_full
                .depth
                .sell
                .into_iter()
                .map(|sell| {
                    let price =
                        convert_price(segment, i32::from_be_bytes(sell.price.try_into().unwrap()));
                    let quantity = i32::from_be_bytes(sell.quantity.try_into().unwrap());
                    let orders = i16::from_be_bytes(sell.orders.try_into().unwrap()) as i32;
                    MarketDepthData {
                        price,
                        quantity,
                        orders,
                    }
                })
                .collect(),
        };
        Self {
            tradable: tradable(segment),
            mode: TickerStreamingMode::Full,
            instrument_token,
            last_price,
            last_traded_quantity,
            average_traded_price,
            volume_traded,
            total_buy_quantity,
            total_sell_quantity,
            ohlc,
            price_change,
            depth,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedDepth {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub depth: MarketDepth,
}

impl From<ExtendedDepthBin> for ExtendedDepth {
    fn from(extended_depth_bin: ExtendedDepthBin) -> Self {
        let instrument_token =
            i32::from_be_bytes(extended_depth_bin.instrument_token.try_into().unwrap());
        let segment = segment(instrument_token);
        let depth = MarketDepth {
            buy: extended_depth_bin
                .depth
                .buy
                .into_iter()
                .map(|buy| {
                    let price =
                        convert_price(segment, i32::from_be_bytes(buy.price.try_into().unwrap()));
                    let quantity = i32::from_be_bytes(buy.quantity.try_into().unwrap());
                    let orders = i32::from_be_bytes(buy.orders.try_into().unwrap());
                    MarketDepthData {
                        price,
                        quantity,
                        orders,
                    }
                })
                .collect(),
            sell: extended_depth_bin
                .depth
                .sell
                .into_iter()
                .map(|sell| {
                    let price =
                        convert_price(segment, i32::from_be_bytes(sell.price.try_into().unwrap()));
                    let quantity = i32::from_be_bytes(sell.quantity.try_into().unwrap());
                    let orders = i32::from_be_bytes(sell.orders.try_into().unwrap());
                    MarketDepthData {
                        price,
                        quantity,
                        orders,
                    }
                })
                .collect(),
        };
        Self {
            tradable: tradable(segment),
            mode: TickerStreamingMode::Full,
            instrument_token,
            depth,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketDepth {
    pub buy: Vec<MarketDepthData>,
    pub sell: Vec<MarketDepthData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketDepthData {
    pub price: f64,
    pub orders: i32,
    pub quantity: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceChange {
    pub change_percent: f64,
    pub absolute_change: f64,
    pub open_change: f64,
    pub open_change_percent: f64,
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum TickerSchemaType {
    Ltp,
    Ltpc,
    QuoteIndex,
    FullIndex,
    Quote,
    Full,
    CompactFull,
    ExtendedDepth,
}

impl TickerSchemaType {
    fn default() -> Self {
        Self::Ltp
    }

    fn from(num: i16) -> Option<Self> {
        match num {
            8 => Some(TickerSchemaType::Ltp),
            12 => Some(TickerSchemaType::Ltpc),
            28 => Some(TickerSchemaType::QuoteIndex),
            32 => Some(TickerSchemaType::FullIndex),
            44 => Some(TickerSchemaType::Quote),
            164 => Some(TickerSchemaType::CompactFull),
            184 => Some(TickerSchemaType::Full),
            492 => Some(TickerSchemaType::ExtendedDepth),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickerSchemaTypeData {
    Ltp(Ltp),
    Ltpc(Ltpc),
    QuoteIndex(QuoteIndex),
    FullIndex(FullIndex),
    Quote(Quote),
    Full(Full),
    CompactFull(CompactFull),
    ExtendedDepth(ExtendedDepth),
}

impl Default for TickerSchemaTypeData {
    fn default() -> Self {
        Self::Ltp(Default::default())
    }
}

impl TickerSchemaTypeData {
    pub fn from(ticker_schema_type: TickerSchemaType) -> Self {
        match ticker_schema_type {
            TickerSchemaType::Ltp => Self::Ltp(Default::default()),
            TickerSchemaType::Ltpc => Self::Ltpc(Default::default()),
            TickerSchemaType::QuoteIndex => Self::QuoteIndex(Default::default()),
            TickerSchemaType::FullIndex => Self::FullIndex(Default::default()),
            TickerSchemaType::Quote => Self::Quote(Default::default()),
            TickerSchemaType::Full => Self::Full(Default::default()),
            TickerSchemaType::CompactFull => Self::CompactFull(Default::default()),
            TickerSchemaType::ExtendedDepth => Self::ExtendedDepth(Default::default()),
        }
    }
    pub fn from_bin(packet_schema_type_data: PacketSchemaTypeData) -> Self {
        match packet_schema_type_data {
            PacketSchemaTypeData::LTP(ltp_bin) => Self::Ltp(Ltp::from(ltp_bin)),
            PacketSchemaTypeData::LTPC(ltpc_bin) => Self::Ltpc(Ltpc::from(ltpc_bin)),
            PacketSchemaTypeData::QUOTEINDEX(quote_index_bin) => {
                Self::QuoteIndex(QuoteIndex::from(quote_index_bin))
            }
            PacketSchemaTypeData::FULLINDEX(full_index_bin) => {
                Self::FullIndex(FullIndex::from(full_index_bin))
            }
            PacketSchemaTypeData::QUOTE(quote_bin) => Self::Quote(Quote::from(quote_bin)),
            PacketSchemaTypeData::FULL(full_bin) => Self::Full(Full::from(full_bin)),
            PacketSchemaTypeData::COMPACTFULL(compact_full_bin) => {
                Self::CompactFull(CompactFull::from(compact_full_bin))
            }
            PacketSchemaTypeData::EXTENDEDDEPTH(extended_depth_bin) => {
                Self::ExtendedDepth(ExtendedDepth::from(extended_depth_bin))
            }
        }
    }
}

pub fn segment(instrument_token: i32) -> i32 {
    instrument_token & 0xFF
}

pub fn is_index(segment: i32) -> bool {
    segment == ExchangeMap::Indices as i32
}

pub fn tradable(segment: i32) -> bool {
    !is_index(segment)
}

// convertPrice converts prices of stocks from paise to rupees
// with varying decimals based on the segment.
pub fn convert_price(segment: i32, price: i32) -> f64 {
    let divisor_val: i32 = match segment {
        segment if segment == ExchangeMap::Cds as i32 => 10_000_000_i32,
        segment if segment == ExchangeMap::Bcd as i32 => 10_000_i32,
        _ => 100_i32,
    };
    price as f64 / divisor_val as f64
}

pub fn calculate_change(last_price: f64, open_price: f64, close_price: f64) -> PriceChange {
    if close_price != 0.0_f64 && open_price != 0.0_f64 {
        return PriceChange {
            change_percent: ((last_price - close_price) * 100.0_f64) / close_price,
            absolute_change: (last_price - close_price),
            open_change: (last_price - open_price),
            open_change_percent: ((last_price - open_price) * 100.0_f64) / open_price,
        };
    } else {
        return PriceChange::default();
    }
}
