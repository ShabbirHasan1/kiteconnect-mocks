use super::common::*;
use super::ticker::*;
use crate::utils::*;
use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ltp {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
}

impl From<LtpBin> for Ltp {
    fn from(ltp_bin: LtpBin) -> Self {
        let token = i32::from_be_bytes(ltp_bin.token.try_into().unwrap());
        let segment = segment(token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(ltp_bin.last_price.try_into().unwrap()),
        );
        Self {
            mode: TickerStreamingMode::Ltp,
            is_tradable: is_tradable(segment),
            token,
            last_price,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ltpc {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    pub close_price: f64,
    pub change: f64,
    pub absolute_change: f64,
    pub tick_change: i8,
}

impl From<LtpcBin> for Ltpc {
    fn from(ltpc_bin: LtpcBin) -> Self {
        let token = i32::from_be_bytes(ltpc_bin.token.try_into().unwrap());
        let segment = segment(token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(ltpc_bin.last_price.try_into().unwrap()),
        );
        let close_price = convert_price(
            segment,
            i32::from_be_bytes(ltpc_bin.close_price.try_into().unwrap()),
        );
        let price_change: PriceChange =
            PriceChange::calculate_close_price_change(last_price, close_price);
        let (change, absolute_change, tick_change) = (
            price_change.change,
            price_change.absolute_change,
            price_change.tick_change,
        );
        Self {
            mode: TickerStreamingMode::Ltpc,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change,
            absolute_change,
            tick_change,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteIndex {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    pub close_price: f64,
    pub change: f64,
    pub absolute_change: f64,
    pub open_change: f64,
    pub open_change_percent: f64,
    pub tick_change: i8,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub change_from_tick_packet: f64,
}

impl From<QuoteIndexBin> for QuoteIndex {
    fn from(quote_index_bin: QuoteIndexBin) -> Self {
        let token = i32::from_be_bytes(quote_index_bin.token.try_into().unwrap());
        let segment = segment(token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(quote_index_bin.last_price.try_into().unwrap()),
        );
        let open_price = convert_price(
            segment,
            i32::from_be_bytes(quote_index_bin.ohlc.open.try_into().unwrap()),
        );
        let high_price = convert_price(
            segment,
            i32::from_be_bytes(quote_index_bin.ohlc.high.try_into().unwrap()),
        );
        let low_price = convert_price(
            segment,
            i32::from_be_bytes(quote_index_bin.ohlc.low.try_into().unwrap()),
        );
        let close_price = convert_price(
            segment,
            i32::from_be_bytes(quote_index_bin.ohlc.close.try_into().unwrap()),
        );
        let change_from_tick_packet =
            i32::from_be_bytes(quote_index_bin.change.try_into().unwrap()) as f64;
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        let (change, absolute_change, open_change, open_change_percent, tick_change) = (
            price_change.change,
            price_change.absolute_change,
            price_change.open_change,
            price_change.open_change_percent,
            price_change.tick_change,
        );
        Self {
            mode: TickerStreamingMode::Quote,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change,
            absolute_change,
            open_change,
            open_change_percent,
            tick_change,
            open_price,
            high_price,
            low_price,
            change_from_tick_packet,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullIndex {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    pub close_price: f64,
    pub change: f64,
    pub absolute_change: f64,
    pub open_change: f64,
    pub open_change_percent: f64,
    pub tick_change: i8,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub exchange_timestamp: DateTime<FixedOffset>,
    pub change_from_tick_packet: f64,
}

impl From<FullIndexBin> for FullIndex {
    fn from(full_index_bin: FullIndexBin) -> Self {
        let token = i32::from_be_bytes(full_index_bin.token.try_into().unwrap());
        let segment = segment(token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(full_index_bin.last_price.try_into().unwrap()),
        );
        let open_price = convert_price(
            segment,
            i32::from_be_bytes(full_index_bin.ohlc.open.try_into().unwrap()),
        );
        let high_price = convert_price(
            segment,
            i32::from_be_bytes(full_index_bin.ohlc.high.try_into().unwrap()),
        );
        let low_price = convert_price(
            segment,
            i32::from_be_bytes(full_index_bin.ohlc.low.try_into().unwrap()),
        );
        let close_price = convert_price(
            segment,
            i32::from_be_bytes(full_index_bin.ohlc.close.try_into().unwrap()),
        );
        let change_from_tick_packet =
            i32::from_be_bytes(full_index_bin.change.try_into().unwrap()) as f64;
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        let (change, absolute_change, open_change, open_change_percent, tick_change) = (
            price_change.change,
            price_change.absolute_change,
            price_change.open_change,
            price_change.open_change_percent,
            price_change.tick_change,
        );
        let exchange_timestamp =
            i32::from_be_bytes(full_index_bin.exchange_timestamp.try_into().unwrap()) as i64;
        Self {
            mode: TickerStreamingMode::Full,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change,
            absolute_change,
            open_change,
            open_change_percent,
            tick_change,
            open_price,
            high_price,
            low_price,
            exchange_timestamp: FixedOffset::east(19800).timestamp(exchange_timestamp, 0),
            change_from_tick_packet,
        }
    }
}

impl Default for FullIndex {
    fn default() -> Self {
        Self {
            exchange_timestamp: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    pub close_price: f64,
    pub change: f64,
    pub absolute_change: f64,
    pub open_change: f64,
    pub open_change_percent: f64,
    pub tick_change: i8,
    pub volume: i32,
    pub last_quantity: i32,
    pub total_buy_quantity: i32,
    pub total_sell_quantity: i32,
    pub average_price: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
}

impl From<QuoteBin> for Quote {
    fn from(quote_bin: QuoteBin) -> Self {
        let token = i32::from_be_bytes(quote_bin.token.try_into().unwrap());
        let segment = segment(token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(quote_bin.last_price.try_into().unwrap()),
        );
        let last_quantity = i32::from_be_bytes(quote_bin.last_quantity.try_into().unwrap());
        let average_price = convert_price(
            segment,
            i32::from_be_bytes(quote_bin.average_price.try_into().unwrap()),
        );
        let volume = i32::from_be_bytes(quote_bin.volume.try_into().unwrap());
        let total_buy_quantity =
            i32::from_be_bytes(quote_bin.total_buy_quantity.try_into().unwrap());
        let total_sell_quantity =
            i32::from_be_bytes(quote_bin.total_sell_quantity.try_into().unwrap());
        let open_price = convert_price(
            segment,
            i32::from_be_bytes(quote_bin.ohlc.open.try_into().unwrap()),
        );
        let high_price = convert_price(
            segment,
            i32::from_be_bytes(quote_bin.ohlc.high.try_into().unwrap()),
        );
        let low_price = convert_price(
            segment,
            i32::from_be_bytes(quote_bin.ohlc.low.try_into().unwrap()),
        );
        let close_price = convert_price(
            segment,
            i32::from_be_bytes(quote_bin.ohlc.close.try_into().unwrap()),
        );
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        let (change, absolute_change, open_change, open_change_percent, tick_change) = (
            price_change.change,
            price_change.absolute_change,
            price_change.open_change,
            price_change.open_change_percent,
            price_change.tick_change,
        );
        Self {
            mode: TickerStreamingMode::Quote,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change,
            absolute_change,
            open_change,
            open_change_percent,
            tick_change,
            volume,
            last_quantity,
            total_buy_quantity,
            total_sell_quantity,
            average_price,
            open_price,
            high_price,
            low_price,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Full {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    pub close_price: f64,
    pub change: f64,
    pub absolute_change: f64,
    pub open_change: f64,
    pub open_change_percent: f64,
    pub tick_change: i8,
    pub volume: i32,
    pub last_quantity: i32,
    pub total_buy_quantity: i32,
    pub total_sell_quantity: i32,
    pub average_price: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub depth: MarketDepth,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub last_traded_time: DateTime<FixedOffset>,
    pub oi: i32,
    pub oi_day_high: i32,
    pub oi_day_low: i32,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub exchange_timestamp: DateTime<FixedOffset>,
}

impl From<FullBin> for Full {
    fn from(full_bin: FullBin) -> Self {
        let token = i32::from_be_bytes(full_bin.token.try_into().unwrap());
        let segment = segment(token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(full_bin.last_price.try_into().unwrap()),
        );
        let last_quantity = i32::from_be_bytes(full_bin.last_quantity.try_into().unwrap());
        let average_price = convert_price(
            segment,
            i32::from_be_bytes(full_bin.average_price.try_into().unwrap()),
        );
        let volume = i32::from_be_bytes(full_bin.volume.try_into().unwrap());
        let total_buy_quantity =
            i32::from_be_bytes(full_bin.total_buy_quantity.try_into().unwrap());
        let total_sell_quantity =
            i32::from_be_bytes(full_bin.total_sell_quantity.try_into().unwrap());
        let open_price = convert_price(
            segment,
            i32::from_be_bytes(full_bin.ohlc.open.try_into().unwrap()),
        );
        let high_price = convert_price(
            segment,
            i32::from_be_bytes(full_bin.ohlc.high.try_into().unwrap()),
        );
        let low_price = convert_price(
            segment,
            i32::from_be_bytes(full_bin.ohlc.low.try_into().unwrap()),
        );
        let close_price = convert_price(
            segment,
            i32::from_be_bytes(full_bin.ohlc.close.try_into().unwrap()),
        );
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        let (change, absolute_change, open_change, open_change_percent, tick_change) = (
            price_change.change,
            price_change.absolute_change,
            price_change.open_change,
            price_change.open_change_percent,
            price_change.tick_change,
        );
        let last_traded_time =
            i32::from_be_bytes(full_bin.last_traded_time.try_into().unwrap()) as i64;
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
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
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
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
        };
        Self {
            mode: TickerStreamingMode::Full,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change,
            absolute_change,
            open_change,
            open_change_percent,
            tick_change,
            volume,
            last_quantity,
            total_buy_quantity,
            total_sell_quantity,
            average_price,
            open_price,
            high_price,
            low_price,
            depth,
            last_traded_time: FixedOffset::east(19800).timestamp(last_traded_time, 0),
            oi,
            oi_day_low,
            oi_day_high,
            exchange_timestamp: FixedOffset::east(19800).timestamp(exchange_timestamp, 0),
        }
    }
}

impl Default for Full {
    fn default() -> Self {
        Self {
            last_traded_time: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            exchange_timestamp: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactFull {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    pub close_price: f64,
    pub change: f64,
    pub absolute_change: f64,
    pub open_change: f64,
    pub open_change_percent: f64,
    pub tick_change: i8,
    pub volume: i32,
    pub last_quantity: i32,
    pub total_buy_quantity: i32,
    pub total_sell_quantity: i32,
    pub average_price: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub depth: MarketDepth,
}

impl From<CompactFullBin> for CompactFull {
    fn from(compact_full: CompactFullBin) -> Self {
        let token = i32::from_be_bytes(compact_full.token.try_into().unwrap());
        let segment = segment(token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(compact_full.last_price.try_into().unwrap()),
        );
        let last_quantity = i32::from_be_bytes(compact_full.last_quantity.try_into().unwrap());
        let average_price = convert_price(
            segment,
            i32::from_be_bytes(compact_full.average_price.try_into().unwrap()),
        );
        let volume = i32::from_be_bytes(compact_full.volume.try_into().unwrap());
        let total_buy_quantity =
            i32::from_be_bytes(compact_full.total_buy_quantity.try_into().unwrap());
        let total_sell_quantity =
            i32::from_be_bytes(compact_full.total_sell_quantity.try_into().unwrap());
        let open_price = convert_price(
            segment,
            i32::from_be_bytes(compact_full.ohlc.open.try_into().unwrap()),
        );
        let high_price = convert_price(
            segment,
            i32::from_be_bytes(compact_full.ohlc.high.try_into().unwrap()),
        );
        let low_price = convert_price(
            segment,
            i32::from_be_bytes(compact_full.ohlc.low.try_into().unwrap()),
        );
        let close_price = convert_price(
            segment,
            i32::from_be_bytes(compact_full.ohlc.close.try_into().unwrap()),
        );
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        let (change, absolute_change, open_change, open_change_percent, tick_change) = (
            price_change.change,
            price_change.absolute_change,
            price_change.open_change,
            price_change.open_change_percent,
            price_change.tick_change,
        );
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
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
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
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
        };
        Self {
            mode: TickerStreamingMode::Full,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change,
            absolute_change,
            open_change,
            open_change_percent,
            tick_change,
            volume,
            last_quantity,
            total_buy_quantity,
            total_sell_quantity,
            average_price,
            open_price,
            high_price,
            low_price,
            depth,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedDepth {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub last_traded_time: DateTime<FixedOffset>,
    pub extended_depth: ExtendedMarketDepth,
}

impl Default for ExtendedDepth {
    fn default() -> Self {
        Self {
            last_traded_time: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            ..Default::default()
        }
    }
}

impl From<ExtendedDepthBin> for ExtendedDepth {
    fn from(extended_depth_bin: ExtendedDepthBin) -> Self {
        let token = i32::from_be_bytes(extended_depth_bin.token.try_into().unwrap());
        let segment = segment(token);
        let last_price = convert_price(
            segment,
            i32::from_be_bytes(extended_depth_bin.last_price.try_into().unwrap()),
        );
        let last_traded_time =
            i32::from_be_bytes(extended_depth_bin.last_traded_time.try_into().unwrap()) as i64;
        let extended_depth = ExtendedMarketDepth {
            buy: extended_depth_bin
                .extended_depth
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
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
            sell: extended_depth_bin
                .extended_depth
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
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
        };
        Self {
            mode: TickerStreamingMode::Full,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            last_traded_time: FixedOffset::east(19800).timestamp(last_traded_time, 0),
            extended_depth,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullExtendedDepth {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    pub close_price: f64,
    pub change: f64,
    pub absolute_change: f64,
    pub open_change: f64,
    pub open_change_percent: f64,
    pub tick_change: i8,
    pub volume: i32,
    pub last_quantity: i32,
    pub total_buy_quantity: i32,
    pub total_sell_quantity: i32,
    pub average_price: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub depth: MarketDepth,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub last_traded_time: DateTime<FixedOffset>,
    pub oi: i32,
    pub oi_day_high: i32,
    pub oi_day_low: i32,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub exchange_timestamp: DateTime<FixedOffset>,
    pub extended_depth: ExtendedMarketDepth,
}

impl Default for FullExtendedDepth {
    fn default() -> Self {
        Self {
            last_traded_time: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            exchange_timestamp: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepth {
    pub buy: [MarketDepthData; 5],
    pub sell: [MarketDepthData; 5],
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedMarketDepth {
    pub buy: [MarketDepthData; 20],
    pub sell: [MarketDepthData; 20],
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepthData {
    pub price: f64,
    pub orders: i32,
    pub quantity: i32,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceChange {
    pub change: f64,
    pub absolute_change: f64,
    pub open_change: f64,
    pub open_change_percent: f64,
    pub tick_change: i8,
}

impl PriceChange {
    pub fn calculate_price_change(last_price: f64, open_price: f64, close_price: f64) -> Self {
        if (open_price, close_price) != (0.0_f64, 0.0_f64) {
            let open_change: f64 = last_price - open_price;
            let open_change_percent: f64 = (open_change * 100.0_f64) / open_price;
            let absolute_change: f64 = last_price - close_price;
            let change: f64 = (absolute_change * 100.0_f64) / close_price;
            let tick_change: i8 = if change > 0.0_f64 { 1 } else { -1 };
            Self {
                change,
                absolute_change,
                open_change,
                open_change_percent,
                tick_change,
            }
        } else {
            Self::default()
        }
    }
    pub fn calculate_open_price_change(last_price: f64, open_price: f64) -> Self {
        if open_price != 0.0_f64 {
            let open_change: f64 = last_price - open_price;
            let open_change_percent: f64 = (open_change * 100.0_f64) / open_price;
            Self {
                change: Default::default(),
                absolute_change: Default::default(),
                open_change,
                open_change_percent,
                tick_change: Default::default(),
            }
        } else {
            Self::default()
        }
    }
    pub fn calculate_close_price_change(last_price: f64, close_price: f64) -> Self {
        if close_price != 0.0_f64 {
            let absolute_change: f64 = last_price - close_price;
            let change: f64 = (absolute_change * 100.0_f64) / close_price;
            let tick_change: i8 = if change > 0.0_f64 { 1 } else { -1 };
            Self {
                change,
                absolute_change,
                open_change: Default::default(),
                open_change_percent: Default::default(),
                tick_change,
            }
        } else {
            Self::default()
        }
    }
}

pub fn segment(token: i32) -> i32 {
    token & 0xFF
}

pub fn is_index(segment: i32) -> bool {
    segment == ExchangeMap::Indices as i32
}

pub fn is_tradable(segment: i32) -> bool {
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
