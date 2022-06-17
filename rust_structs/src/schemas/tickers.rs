use std::sync::Arc;

use super::common::*;
use super::ticker::*;
use crate::utils::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ltp {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
}

impl From<&[u8]> for Ltp {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(
            buf.remaining(),
            8,
            "Required Size of Ltp Buffer is 8 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let token = buf.get_i32();
        let segment = get_segment(token);
        assert_eq!(
            buf.remaining(),
            4,
            "Required Size of Ltp Buffer After Parsing Token<i32> is 4 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let last_price = convert_price(segment.into(), buf.get_i32());
        assert!(
            buf.has_remaining(),
            "After Parsing LastPrice<i32> Ltp Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes",
            buf.remaining()
        );
        Self {
            mode: TickerStreamingMode::Ltp,
            is_tradable: is_tradable(segment),
            token,
            last_price,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl From<&[u8]> for Ltpc {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(
            buf.remaining(),
            12,
            "Required Size of Ltpc Buffer is 12 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let token = buf.get_i32();
        let segment = get_segment(token);
        assert_eq!(
            buf.remaining(),
            8,
            "Required Size of Ltpc Buffer After Parsing Token<i32> is 8 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let last_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            4,
            "Required Size of Ltpc Buffer After Parsing LastPrice<i32> is 4 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let close_price = convert_price(segment.into(), buf.get_i32());
        assert!(
            buf.has_remaining(),
            "After Parsing ClosePrice<i32> Ltpc Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes",
            buf.remaining()
        );
        let price_change: PriceChange =
            PriceChange::calculate_close_price_change(last_price, close_price);
        Self {
            mode: TickerStreamingMode::Ltpc,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change: price_change.change,
            absolute_change: price_change.absolute_change,
            tick_change: price_change.tick_change,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl From<&[u8]> for QuoteIndex {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(
            buf.remaining(),
            28,
            "Required Size of QuoteIndex Buffer is 28 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let token = buf.get_i32();
        let segment = get_segment(token);
        assert_eq!(buf.remaining(), 24, "Required Size of QuoteIndex Buffer After Parsing Token<i32> is 24 Bytes, But Found {} Bytes", buf.remaining());
        let last_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 20, "Required Size of QuoteIndex Buffer After Parsing LastPrice<i32> is 20 Bytes, But Found {} Bytes", buf.remaining());
        let high_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 16, "Required Size of QuoteIndex Buffer After Parsing HighPrice<i32> is 16 Bytes, But Found {} Bytes", buf.remaining());
        let low_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 12, "Required Size of QuoteIndex Buffer After Parsing LowPrice<i32> is 12 Bytes, But Found {} Bytes", buf.remaining());
        let open_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 8, "Required Size of QuoteIndex Buffer After Parsing OpenPrice<i32> is 8 Bytes, But Found {} Bytes", buf.remaining());
        let close_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 4, "Required Size of QuoteIndex Buffer After Parsing ClosePrice<i32> is 4 Bytes, But Found {} Bytes", buf.remaining());
        let change_from_tick_packet = buf.get_i32() as f64;
        assert!(buf.has_remaining(), "After Parsing ChangeFromTickPacket<i32> QuoteIndex Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        Self {
            mode: TickerStreamingMode::Quote,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change: price_change.change,
            absolute_change: price_change.absolute_change,
            open_change: price_change.open_change,
            open_change_percent: price_change.open_change_percent,
            tick_change: price_change.tick_change,
            open_price,
            high_price,
            low_price,
            change_from_tick_packet,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl From<&[u8]> for FullIndex {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(
            buf.remaining(),
            32,
            "Required Size of FullIndex Buffer is 32 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let token = buf.get_i32();
        let segment = get_segment(token);
        assert_eq!(buf.remaining(), 28, "Required Size of FullIndex Buffer After Parsing Token<i32> is 28 Bytes, But Found {} Bytes", buf.remaining());
        let last_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 24, "Required Size of FullIndex Buffer After Parsing LastPrice<i32> is 24 Bytes, But Found {} Bytes", buf.remaining());
        let high_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 20, "Required Size of FullIndex Buffer After Parsing HighPrice<i32> is 20 Bytes, But Found {} Bytes", buf.remaining());
        let low_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 16, "Required Size of FullIndex Buffer After Parsing LowPrice<i32> is 16 Bytes, But Found {} Bytes", buf.remaining());
        let open_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 12, "Required Size of FullIndex Buffer After Parsing OpenPrice<i32> is 12 Bytes, But Found {} Bytes", buf.remaining());
        let close_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 8, "Required Size of FullIndex Buffer After Parsing ClosePrice<i32> is 8 Bytes, But Found {} Bytes", buf.remaining());
        let change_from_tick_packet = buf.get_i32() as f64;
        assert_eq!(buf.remaining(), 4, "Required Size of FullIndex Buffer After Parsing ChangeFromTickPacket<i32> is 4 Bytes, But Found {} Bytes", buf.remaining());
        let exchange_timestamp = buf.get_i32() as i64;
        assert!(buf.has_remaining(), "After Parsing ExchangeTimestamp<i32> FullIndex Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        Self {
            mode: TickerStreamingMode::Full,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change: price_change.change,
            absolute_change: price_change.absolute_change,
            open_change: price_change.open_change,
            open_change_percent: price_change.open_change_percent,
            tick_change: price_change.tick_change,
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl From<&[u8]> for Quote {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(
            buf.remaining(),
            44,
            "Required Size of Quote Buffer is 44 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let token = buf.get_i32();
        let segment = get_segment(token);
        assert_eq!(buf.remaining(), 40, "Required Size of Quote Buffer After Parsing Token<i32> is 40 Bytes, But Found {} Bytes", buf.remaining());
        let last_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 36, "Required Size of Quote Buffer After Parsing LastPrice<i32> is 36 Bytes, But Found {} Bytes", buf.remaining());
        let last_quantity = buf.get_i32();
        assert_eq!(buf.remaining(), 32, "Required Size of Quote Buffer After Parsing LastQuantity<i32> is 320 Bytes, But Found {} Bytes", buf.remaining());
        let average_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 28, "Required Size of Quote Buffer After Parsing AveragePrice<i32> is 28 Bytes, But Found {} Bytes", buf.remaining());
        let volume = buf.get_i32();
        assert_eq!(buf.remaining(), 24, "Required Size of Quote Buffer After Parsing Volume<i32> is 24 Bytes, But Found {} Bytes", buf.remaining());
        let total_buy_quantity = buf.get_i32();
        assert_eq!(buf.remaining(), 20, "Required Size of Quote Buffer After Parsing TotalBuyQuantity<i32> is 20 Bytes, But Found {} Bytes", buf.remaining());
        let total_sell_quantity = buf.get_i32();
        assert_eq!(buf.remaining(), 16, "Required Size of Quote Buffer After Parsing TotalSellQuantity<i32> is 16 Bytes, But Found {} Bytes", buf.remaining());
        let open_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 12, "Required Size of Quote Buffer After Parsing OpenPrice<i32> is 12 Bytes, But Found {} Bytes", buf.remaining());
        let high_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 8, "Required Size of Quote Buffer After Parsing HighPrice<i32> is 8 Bytes, But Found {} Bytes", buf.remaining());
        let low_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(buf.remaining(), 4, "Required Size of Quote Buffer After Parsing LowPrice<i32> is 4 Bytes, But Found {} Bytes", buf.remaining());
        let close_price = convert_price(segment.into(), buf.get_i32());
        assert!(buf.has_remaining(), "After Parsing ClosePrice<i32> Quote Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        Self {
            mode: TickerStreamingMode::Quote,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change: price_change.change,
            absolute_change: price_change.absolute_change,
            open_change: price_change.open_change,
            open_change_percent: price_change.open_change_percent,
            tick_change: price_change.tick_change,
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl From<&[u8]> for Full {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(
            buf.remaining(),
            184,
            "Required Size of Full Buffer is 184 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let token = buf.get_i32();
        let segment = get_segment(token);
        assert_eq!(
            buf.remaining(),
            180,
            "Required Size of Full Buffer After Parsing Token<i32> is 180 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let last_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            176,
            "Required Size of Full Buffer After Parsing LastPrice<i32> is 176 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let last_quantity = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            172,
            "Required Size of Full Buffer After Parsing LastQuantity<i32> is 172 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let average_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            168,
            "Required Size of Full Buffer After Parsing AveragePrice<i32> is 168 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let volume = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            164,
            "Required Size of Full Buffer After Parsing Volume<i32> is 164 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let total_buy_quantity = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            160,
            "Required Size of Full Buffer After Parsing TotalBuyQuantity<i32> is 160 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let total_sell_quantity = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            156,
            "Required Size of Full Buffer After Parsing TotalSellQuantity<i32> is 156 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let open_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            152,
            "Required Size of Full Buffer After Parsing OpenPrice<i32> is 152 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let high_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            148,
            "Required Size of Full Buffer After Parsing HighPrice<i32> is 148 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let low_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            144,
            "Required Size of Full Buffer After Parsing LowPrice<i32> is 144 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let close_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            140,
            "Required Size of Full Buffer After Parsing ClosePrice<i32> is 140 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let last_traded_time = buf.get_i32() as i64;
        assert_eq!(
            buf.remaining(),
            136,
            "Required Size of Full Buffer After Parsing LastTradedTime<i32> is 136 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let oi = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            132,
            "Required Size of Full Buffer After Parsing OI<i32> is 132 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let oi_day_low = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            128,
            "Required Size of Full Buffer After Parsing OIDayLow<i32> is 128 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let oi_day_high = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            124,
            "Required Size of Full Buffer After Parsing OIDayHigh<i32> is 124 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let exchange_timestamp = buf.get_i32() as i64;
        assert_eq!(
            buf.remaining(),
            120,
            "Required Size of Full Buffer After Parsing ExchangeTimestamp<i32> is 120 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let depth = MarketDepth {
            buy: [0_u8; 5]
                .into_iter()
                .enumerate()
                .map(|(idx, _)| {
                    assert_eq!(
                        buf.remaining(),
                        120 - (idx * 12),
                        "Required Size of Full Buffer After Parsing\nMarketDepth {{\nbuy: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 120 - (idx * 12),
                        buf.remaining()
                    );
                    let market_depth_data = MarketDepthData {
                        quantity: buf.get_i32(),
                        price: convert_price(segment.into(), buf.get_i32()),
                        orders: buf.get_i16() as i32,
                    };
                    assert_eq!(
                        buf.remaining(),
                        120 - (idx + 1) * 10,
                        "Required Size of Full Buffer After Parsing\nMarketDepth {{\nbuy: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 120 - (idx + 1) * 10,
                        buf.remaining()
                    );
                    buf.get_i16();
                    assert_eq!(
                        buf.remaining(),
                        120 - (idx + 1) * 12,
                        "Required Size of Full Buffer After Parsing Padding<i16> is {} Bytes, But Found {} Bytes",
                        120 - (idx + 1) * 12,
                        buf.remaining()
                    );
                    market_depth_data
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
            sell: [0_u8; 5]
                .into_iter()
                .enumerate()
                .map(|(idx, _)| {
                    assert_eq!(
                        buf.remaining(),
                        60 - (idx * 12),
                        "Required Size of Full Buffer Before Parsing\nMarketDepth {{\nsell: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 60 - (idx * 12),
                        buf.remaining()
                    );
                    let market_depth_data = MarketDepthData {
                        quantity: buf.get_i32(),
                        price: convert_price(segment.into(), buf.get_i32()),
                        orders: buf.get_i16() as i32,
                    };
                    assert_eq!(
                        buf.remaining(),
                        60 - (idx + 1) * 10,
                        "Required Size of Full Buffer After Parsing\nMarketDepth {{\nsell: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 60 - (idx + 1) * 10,
                        buf.remaining()
                    );
                    buf.get_i16();
                    assert_eq!(
                        buf.remaining(),
                        60 - (idx + 1) * 12,
                        "Required Size of Full Buffer After Parsing Padding<i16> is {} Bytes, But Found {} Bytes",
                        60 - (idx + 1) * 12,
                        buf.remaining()
                    );
                    market_depth_data
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
        };
        assert!(buf.has_remaining(), "After Parsing Depth<MarketDepth> Full Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        Self {
            mode: TickerStreamingMode::Full,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change: price_change.change,
            absolute_change: price_change.absolute_change,
            open_change: price_change.open_change,
            open_change_percent: price_change.open_change_percent,
            tick_change: price_change.tick_change,
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl From<&[u8]> for CompactFull {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(
            buf.remaining(),
            164,
            "Required Size of CompactFull Buffer is 164 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let token = buf.get_i32();
        let segment = get_segment(token);
        assert_eq!(
            buf.remaining(),
            160,
            "Required Size of CompactFull Buffer After Parsing Token<i32> is 160 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let last_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            156,
            "Required Size of CompactFull Buffer After Parsing LastPrice<i32> is 156 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let last_quantity = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            152,
            "Required Size of CompactFull Buffer After Parsing LastQuantity<i32> is 152 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let average_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            148,
            "Required Size of CompactFull Buffer After Parsing AveragePrice<i32> is 148 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let volume = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            144,
            "Required Size of CompactFull Buffer After Parsing Volume<i32> is 144 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let total_buy_quantity = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            140,
            "Required Size of CompactFull Buffer After Parsing TotalBuyQuantity<i32> is 140 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let total_sell_quantity = buf.get_i32();
        assert_eq!(
            buf.remaining(),
            136,
            "Required Size of CompactFull Buffer After Parsing TotalSellQuantity<i32> is 136 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let open_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            132,
            "Required Size of CompactFull Buffer After Parsing OpenPrice<i32> is 132 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let high_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            128,
            "Required Size of CompactFull Buffer After Parsing HighPrice<i32> is 128 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let low_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            124,
            "Required Size of CompactFull Buffer After Parsing LowPrice<i32> is 124 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let close_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            120,
            "Required Size of CompactFull Buffer After Parsing ClosePrice<i32> is 120 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        let depth = MarketDepth {
            buy: [0_u8; 5]
                .into_iter()
                .enumerate()
                .map(|(idx, _)| {
                    assert_eq!(
                        buf.remaining(),
                        120 - (idx * 12),
                        "Required Size of CompactFull Buffer Before Parsing\nMarketDepth {{\nbuy: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 120 - (idx * 12),
                        buf.remaining()
                    );
                    let market_depth_data = MarketDepthData {
                        quantity: buf.get_i32(),
                        price: convert_price(segment.into(), buf.get_i32()),
                        orders: buf.get_i16() as i32,
                    };
                    assert_eq!(
                        buf.remaining(),
                        120 - (idx + 1) * 10,
                        "Required Size of CompactFull Buffer After Parsing\nMarketDepth {{\nbuy: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 120 - (idx + 1) * 10,
                        buf.remaining()
                    );
                    buf.get_i16();
                    assert_eq!(
                        buf.remaining(),
                        120 - (idx + 1) * 12,
                        "Required Size of CompactFull Buffer After Parsing Padding<i16> is {} Bytes, But Found {} Bytes",
                        120 - (idx + 1) * 12,
                        buf.remaining()
                    );
                    market_depth_data
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
            sell: [0_u8; 5]
                .into_iter()
                .enumerate()
                .map(|(idx, _)| {
                    assert_eq!(
                        buf.remaining(),
                        60 - (idx * 12),
                        "Required Size of CompactFull Buffer Before Parsing\nMarketDepth {{\nsell: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 60 - (idx * 12),
                        buf.remaining()
                    );
                    let market_depth_data = MarketDepthData {
                        quantity: buf.get_i32(),
                        price: convert_price(segment.into(), buf.get_i32()),
                        orders: buf.get_i16() as i32,
                    };
                    assert_eq!(
                        buf.remaining(),
                        60 - (idx + 1) * 10,
                        "Required Size of CompactFull Buffer After Parsing\nMarketDepth {{\nsell: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 60 - (idx + 1) * 10,
                        buf.remaining()
                    );
                    buf.get_i16();
                    assert_eq!(
                        buf.remaining(),
                        60 - (idx + 1) * 12,
                        "Required Size of CompactFull Buffer After Parsing Padding<i16> is {} Bytes, But Found {} Bytes",
                        60 - (idx + 1) * 12,
                        buf.remaining()
                    );
                    market_depth_data
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
        };
        assert!(buf.has_remaining(), "After Parsing Depth<MarketDepth> CompactFull Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
        Self {
            mode: TickerStreamingMode::Full,
            is_tradable: is_tradable(segment),
            token,
            last_price,
            close_price,
            change: price_change.change,
            absolute_change: price_change.absolute_change,
            open_change: price_change.open_change,
            open_change_percent: price_change.open_change_percent,
            tick_change: price_change.tick_change,
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl From<&[u8]> for ExtendedDepth {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(
            buf.remaining(),
            492,
            "Required Size of ExtendedDepth Buffer is 492 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let token = buf.get_i32();
        let segment = segment(token);
        assert_eq!(
            buf.remaining(),
            488,
            "Required Size of ExtendedDepth Buffer After Parsing Token<i32> is 488 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let last_price = convert_price(segment.into(), buf.get_i32());
        assert_eq!(
            buf.remaining(),
            484,
            "Required Size of ExtendedDepth Buffer After Parsing LastPrice<i32> is 484 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let last_traded_time = buf.get_i32() as i64;
        assert_eq!(
            buf.remaining(),
            480,
            "Required Size of ExtendedDepth Buffer After Parsing LastTradedTime<i32> is 480 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let extended_depth = ExtendedMarketDepth {
            buy: [0_u8; 20]
                .into_iter()
                .enumerate()
                .map(|(idx, _)| {
                    assert_eq!(
                        buf.remaining(),
                        480 - (idx * 12),
                        "Required Size of ExtendedDepth Buffer Before Parsing\nExtendedMarketDepth {{\nbuy: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 480 - (idx * 12),
                        buf.remaining()
                    );
                    let market_depth_data = MarketDepthData {
                        quantity: buf.get_i32(),
                        price: convert_price(segment.into(), buf.get_i32()),
                        orders: buf.get_i32(),
                    };
                    assert_eq!(
                        buf.remaining(),
                        480 - (idx + 1) * 12,
                        "Required Size of ExtendedDepth Buffer After Parsing\nExtendedMarketDepth {{\nbuy: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 480 - (idx + 1) * 12,
                        buf.remaining()
                    );
                    market_depth_data
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
            sell: [0_u8; 20]
                .into_iter()
                .enumerate()
                .map(|(idx, _)| {
                    assert_eq!(
                        buf.remaining(),
                        240 - (idx * 12),
                        "Required Size of Full Buffer Before Parsing\nExtendedMarketDepth {{\nbuy: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 240 - (idx * 12),
                        buf.remaining()
                    );
                    let market_depth_data = MarketDepthData {
                        quantity: buf.get_i32(),
                        price: convert_price(segment.into(), buf.get_i32()),
                        orders: buf.get_i32(),
                    };
                    assert_eq!(
                        buf.remaining(),
                        240 - (idx + 1) * 12,
                        "Required Size of Full Buffer After Parsing\nExtendedMarketDepth {{\nsell: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 240 - (idx + 1) * 12,
                        buf.remaining()
                    );
                    market_depth_data
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
        };
        assert!(buf.has_remaining(), "After Parsing Depth<MarketDepth> ExtendedDepth Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl FullExtendedDepth {
    fn from(full: Full, extended_depth: ExtendedDepth) -> Self {
        assert_eq!(full.token, extended_depth.token, "Error Producing FullExtendedDepth Due To Token Mis-Match Between Provided Full And ExtendedDepth");
        Self {
            mode: TickerStreamingMode::Full,
            is_tradable: full.is_tradable,
            token: full.token,
            last_price: full.last_price,
            close_price: full.close_price,
            change: full.change,
            absolute_change: full.absolute_change,
            open_change: full.open_change,
            open_change_percent: full.open_change_percent,
            tick_change: full.tick_change,
            volume: full.volume,
            last_quantity: full.last_quantity,
            total_buy_quantity: full.total_buy_quantity,
            total_sell_quantity: full.total_sell_quantity,
            average_price: full.average_price,
            open_price: full.open_price,
            high_price: full.high_price,
            low_price: full.low_price,
            depth: full.depth,
            last_traded_time: full.last_traded_time,
            oi: full.oi,
            oi_day_low: full.oi_day_low,
            oi_day_high: full.oi_day_high,
            exchange_timestamp: full.exchange_timestamp,
            extended_depth: extended_depth.extended_depth,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepth {
    pub buy: [MarketDepthData; 5],
    pub sell: [MarketDepthData; 5],
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedMarketDepth {
    pub buy: [MarketDepthData; 20],
    pub sell: [MarketDepthData; 20],
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepthData {
    pub price: f64,
    pub orders: i32,
    pub quantity: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
    pub tickers: i16,
    pub tickers_data: Vec<TickersData>,
}

impl TryFrom<&[u8]> for Tickers {
    type Error = &'static str;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let tickers = buf.get_i16();
        let tickers_data = vec![0_u8; tickers as usize]
            .iter()
            .map(|_| TickersData::try_from(buf).unwrap())
            .collect::<Vec<TickersData>>();
        Ok(Self {
            tickers,
            tickers_data,
        })
    }
}

impl TryFrom<(&[u8], &[u8])> for Tickers {
    type Error = &'static str;

    fn try_from(full_extended_buf: (&[u8], &[u8])) -> Result<Self, Self::Error> {
        let (full_buf, extended_buf) = full_extended_buf;
        let tickers = full_buf.get_i16();
        let tickers_ext = extended_buf.get_i16();
        assert_eq!(tickers, tickers_ext, "Error Producing Tickers Due To Number of Tickers Mis-Match Between Full And Extended !");
        let tickers_data = vec![0_u8; tickers as usize]
            .iter()
            .map(|_| TickersData::try_from((full_buf, extended_buf)).unwrap())
            .collect::<Vec<TickersData>>();
        Ok(Self {
            tickers,
            tickers_data,
        })
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickersData {
    pub ticker_type: TickerType,
    pub ticker_type_data: TickerTypeData,
}

impl TryFrom<&[u8]> for TickersData {
    type Error = &'static str;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let ticker_type = TickerType::try_from(buf.get_i16() as usize)?;
        let ticker_type_data = TickerTypeData::try_from(buf)?;
        Ok(Self {
            ticker_type,
            ticker_type_data,
        })
    }
}

impl TryFrom<(&[u8], &[u8])> for TickersData {
    type Error = &'static str;

    fn try_from(full_extended_buf: (&[u8], &[u8])) -> Result<Self, Self::Error> {
        let (full_buf, extended_buf) = full_extended_buf;
        let ticker_type = TickerType::try_from(full_buf.get_i16() as usize)?;
        let ticker_type_ext = TickerType::try_from(extended_buf.get_i16() as usize)?;
        let ticker_type_data = TickerTypeData::try_from((full_buf, extended_buf))?;
        Ok(Self {
            ticker_type,
            ticker_type_data,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum TickerType {
    Ltp,
    Ltpc,
    QuoteIndex,
    FullIndex,
    Quote,
    Full,
    CompactFull,
    ExtendedDepth,
    FullExtendedDepth,
}

impl Default for TickerType {
    fn default() -> Self {
        TickerType::Ltp
    }
}

impl TryFrom<usize> for TickerType {
    type Error = &'static str;
    fn try_from(idx: usize) -> Result<Self, Self::Error> {
        match idx {
            8 => Ok(Self::Ltp),
            12 => Ok(Self::Ltpc),
            28 => Ok(Self::QuoteIndex),
            32 => Ok(Self::FullIndex),
            44 => Ok(Self::Quote),
            164 => Ok(Self::CompactFull),
            184 => Ok(Self::Full),
            492 => Ok(Self::ExtendedDepth),
            676 => Ok(Self::FullExtendedDepth),
            _ => Err("TickerType Only Accepts 8, 12, 28, 32, 44, 164, 184, 492, 676 !"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickerTypeData {
    Ltp(Ltp),
    Ltpc(Ltpc),
    QuoteIndex(QuoteIndex),
    FullIndex(FullIndex),
    Quote(Quote),
    Full(Full),
    CompactFull(CompactFull),
    ExtendedDepth(ExtendedDepth),
    FullExtendedDepth(FullExtendedDepth),
}

impl Default for TickerTypeData {
    fn default() -> Self {
        Self::Ltp(Default::default())
    }
}

impl TryFrom<TickerType> for TickerTypeData {
    type Error = &'static str;
    fn try_from(ticker_type: TickerType) -> Result<Self, Self::Error> {
        match ticker_type {
            TickerType::Ltp => Ok(Self::Ltp(Default::default())),
            TickerType::Ltpc => Ok(Self::Ltpc(Default::default())),
            TickerType::QuoteIndex => Ok(Self::QuoteIndex(Default::default())),
            TickerType::FullIndex => Ok(Self::FullIndex(Default::default())),
            TickerType::Quote => Ok(Self::Quote(Default::default())),
            TickerType::Full => Ok(Self::Full(Default::default())),
            TickerType::CompactFull => Ok(Self::CompactFull(Default::default())),
            TickerType::ExtendedDepth => Ok(Self::ExtendedDepth(Default::default())),
            TickerType::FullExtendedDepth => Ok(Self::FullExtendedDepth(Default::default())),
            _ => Err("TickerTypeData Only Accepts TickerType::Ltp, Ltpc, QuoteIndex, FullIndex, Quote, Full, CompactFull, ExtendedDepth, FullExtendedDepth !"),
        }
    }
}

impl TryFrom<&[u8]> for TickerTypeData {
    type Error = &'static str;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        match TickerType::try_from(buf.remaining())? {
            TickerType::Ltp => Ok(TickerTypeData::from(Ltp::from(buf))),
            TickerType::Ltpc => Ok(TickerTypeData::from(Ltpc::from(buf))),
            TickerType::QuoteIndex => Ok(TickerTypeData::from(QuoteIndex::from(buf))),
            TickerType::FullIndex => Ok(TickerTypeData::from(FullIndex::from(buf))),
            TickerType::Quote => Ok(TickerTypeData::from(Quote::from(buf))),
            TickerType::Full => Ok(TickerTypeData::from(Full::from(buf))),
            TickerType::CompactFull => Ok(TickerTypeData::from(CompactFull::from(buf))),
            TickerType::ExtendedDepth => Ok(TickerTypeData::from(ExtendedDepth::from(buf))),
            _ => Err("TickerTypeData Only Accepts TickerType::Ltp, Ltpc, QuoteIndex, FullIndex, Quote, Full, CompactFull, ExtendedDepth, FullExtendedDepth !"),
        }
    }
}

impl TryFrom<(&[u8], &[u8])> for TickerTypeData {
    type Error = &'static str;
    fn try_from(full_extended_buf: (&[u8], &[u8])) -> Result<Self, Self::Error> {
        let (full_buf, extended_buf) = full_extended_buf;
        match TickerType::try_from(full_buf.remaining() + extended_buf.remaining())? {
            TickerType::FullExtendedDepth => Ok(TickerTypeData::from(FullExtendedDepth::from(Full::from(full_buf), ExtendedDepth::from(extended_buf)))),
            _ => Err("TickerTypeData Only Accepts TickerType::Ltp, Ltpc, QuoteIndex, FullIndex, Quote, Full, CompactFull, ExtendedDepth, FullExtendedDepth !"),
        }
    }
}

impl From<Ltp> for TickerTypeData {
    fn from(ltp: Ltp) -> Self {
        Self::Ltp(ltp)
    }
}

impl From<Ltpc> for TickerTypeData {
    fn from(ltpc: Ltpc) -> Self {
        Self::Ltpc(ltpc)
    }
}

impl From<QuoteIndex> for TickerTypeData {
    fn from(quote_index: QuoteIndex) -> Self {
        Self::QuoteIndex(quote_index)
    }
}

impl From<FullIndex> for TickerTypeData {
    fn from(full_index: FullIndex) -> Self {
        Self::FullIndex(full_index)
    }
}

impl From<Quote> for TickerTypeData {
    fn from(quote: Quote) -> Self {
        Self::Quote(quote)
    }
}

impl From<Full> for TickerTypeData {
    fn from(full: Full) -> Self {
        Self::Full(full)
    }
}

impl From<CompactFull> for TickerTypeData {
    fn from(compact_full: CompactFull) -> Self {
        Self::CompactFull(compact_full)
    }
}

impl From<ExtendedDepth> for TickerTypeData {
    fn from(extended_depth: ExtendedDepth) -> Self {
        Self::ExtendedDepth(extended_depth)
    }
}

impl From<FullExtendedDepth> for TickerTypeData {
    fn from(full_extended_depth: FullExtendedDepth) -> Self {
        Self::FullExtendedDepth(full_extended_depth)
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

pub fn get_segment(token: i32) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    // use std::{
    //     fs::{File, OpenOptions},
    //     io::{Cursor, Seek, SeekFrom},
    // };
    #[test]
    fn test_tickers_raw() -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("kite_tickers.json")?;
        if let Ok(lines) = read_lines("./custom_mock_files/ws.packet") {
            for line in lines {
                if let Ok(line) = line {
                    // let data = &base64::decode(data).unwrap()[..];
                    let data = base64::decode(line.as_bytes()).unwrap();
                    // println!("{}", data.len());
                    // println!("{:?}", &data);
                    if data.len() >= 2 {
                        // println!("{}", serde_json::to_string_pretty(&tick_data).unwrap());
                        // println!(
                        //     "{}",
                        //     serde_json::to_string_pretty(&BinaryKiteTickerMessage::from1(data))
                        //         .unwrap()
                        // );
                        // println!("{:#?}", &ticker_data);
                        serde_json::to_writer_pretty(
                            &file,
                            &BinaryKiteTickerMessage::from_cursor(data),
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    // #[test]
    // fn test_tickers_raw_single() -> Result<(), Box<dyn std::error::Error>> {
    //     let data =
    //         b"AAQACAAD+AkANR9gALgAnzoCAAHViAAAABkAAdQLAANHjQAEGD4AAHxRAAHFAgACDWQAAY7ZAAGWaGKhpZkAA37sAAOgtgADTQVioaWbAAAAGQAB1ZIAAQAAAAAAfQAB1Y0AAQAAAAAAGQAB1YgAAQAAAAAAMgAB1YMAAQAAAAAASwAB1X4AAgAAAAAAGQAB1q8AAQAAAAAAGQAB1tcAAQAAAAAAMgAB1uEAAQAAAAAAyAAB1uYAAgAAAAAAGQAB1usAAQAAALgAnzkCAAAAaQAAABkAAAEcAckZrQAWhBAABWLoAAABvQAAAfQAAABaAAAD1GKhpZwAQvKsAE817gA1P0BioaWcAABhwQAAAGQAJgAAAACQ7AAAAF8AOQAAAAA2MwAAAFoAJwAAAAAbJgAAAFUAFwAAAAA0PwAAAFAAHAAAAAAhAgAAAGkALgAAAABOhAAAAG4AOQAAAAAZlgAAAHMAGgAAAAAZ4QAAAHgAFQAAAAAPuQAAAH0AEAAAAAwANx8BAAAffAAAH0U=";
    //     let data = base64::decode(data).unwrap();
    //     // println!("{}", data.len());
    //     // println!("{:?}", &data);
    //     if data.len() > 2 {
    //         // println!("{}", serde_json::to_string_pretty(&tick_data).unwrap());
    //         println!(
    //             "{}",
    //             serde_json::to_string_pretty(&BinaryKiteTickerMessage::from_cursor(data)).unwrap()
    //         );
    //         // println!("{:#?}", &BinaryKiteTickerMessage::from(data));
    //         // println!("{:#?}", &ticker_data);
    //         // serde_json::to_writer_pretty(&File::create("kite_ticker.json")?, &ticker_data)?;
    //     }
    //     Ok(())
    // }
    // #[test]
    // fn test_ticker_three_thousand_quote_message() -> Result<(), Box<dyn std::error::Error>> {
    //     let file = OpenOptions::new()
    //         .write(true)
    //         .append(true)
    //         .create(true)
    //         .open("kite_ticker_three_thousand_quote_message.json")?;
    //     if let Ok(lines) = read_lines("./custom_mock_files/ws_three_thousand_quote_message.packet")
    //     {
    //         for line in lines {
    //             if let Ok(line) = line {
    //                 // let data = &base64::decode(data).unwrap()[..];
    //                 let data = base64::decode(line.as_bytes()).unwrap();
    //                 // println!("{}", data.len());
    //                 // println!("{:?}", &data);
    //                 if data.len() >= 2 {
    //                     let ticker_data = BinaryKiteTickerMessage::from_splits(data);
    //                     // println!("{}", serde_json::to_string_pretty(&ticker_data).unwrap());
    //                     // println!("{:#?}", &ticker_data);
    //                     // serde_json::to_writer_pretty(
    //                     //     &file,
    //                     //     &ticker_data,
    //                     // )?;
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    // #[test]
    // fn test_ticker_extended_depth() -> Result<(), Box<dyn std::error::Error>> {
    //     let file = OpenOptions::new()
    //         .write(true)
    //         .append(true)
    //         .create(true)
    //         .open("kite_ticker_extended_depth.json")?;
    //     if let Ok(lines) = read_lines("./custom_mock_files/ws_extended_depth.packet") {
    //         for line in lines {
    //             if let Ok(line) = line {
    //                 // let data = &base64::decode(data).unwrap()[..];
    //                 let data = base64::decode(line.as_bytes()).unwrap();
    //                 // println!("{}", data.len());
    //                 // println!("{:?}", &data);
    //                 if data.len() >= 2 {
    //                     let ticker_data = BinaryKiteTickerMessage::from_cursor(data);
    //                     // println!("{}", serde_json::to_string_pretty(&ticker_data).unwrap());
    //                     // println!("{:#?}", &ticker_data);
    //                     serde_json::to_writer_pretty(&file, &ticker_data)?;
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    // #[test]
    // fn test_ticker_extended_depth_one() -> Result<(), Box<dyn std::error::Error>> {
    //     let file = OpenOptions::new()
    //         .write(true)
    //         .append(true)
    //         .create(true)
    //         .open("kite_ticker_extended_depth_one.json")?;
    //     if let Ok(lines) = read_lines("./custom_mock_files/ws_extended_depth_20.packet") {
    //         for line in lines {
    //             if let Ok(line) = line {
    //                 // let data = &base64::decode(data).unwrap()[..];
    //                 let data = base64::decode(line.as_bytes()).unwrap();
    //                 // println!("{}", data.len());
    //                 // println!("{:?}", &data);
    //                 if data.len() >= 2 {
    //                     let ticker_data = BinaryKiteTickerMessage::from_cursor(data);
    //                     println!("{}", serde_json::to_string(&ticker_data).unwrap());
    //                     // println!("{:#?}", &ticker_data);
    //                     // serde_json::to_writer_pretty(&file, &ticker_data)?;
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    // #[test]
    // fn test_ticker_three_thousand_extended_quote_message() -> Result<(), Box<dyn std::error::Error>>
    // {
    //     let file = OpenOptions::new()
    //         .write(true)
    //         .append(true)
    //         .create(true)
    //         .open("kite_ticker_three_thousand_extended_quote_message.json")?;
    //     if let Ok(lines) =
    //         read_lines("./custom_mock_files/ws_three_thousand_extended_quote_message.packet")
    //     {
    //         for line in lines {
    //             if let Ok(line) = line {
    //                 // let data = &base64::decode(data).unwrap()[..];
    //                 let data = base64::decode(line.as_bytes()).unwrap();
    //                 // println!("{}", data.len());
    //                 // println!("{:?}", &data);
    //                 if data.len() >= 2 {
    //                     let ticker_data = BinaryKiteTickerMessage::from_cursor(data);
    //                     // println!("{}", serde_json::to_string_pretty(&ticker_data).unwrap());
    //                     // println!("{:#?}", &ticker_data);
    //                     // serde_json::to_writer_pretty(
    //                     //     &file,
    //                     //     &ticker_data,
    //                     // )?;
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }
}
