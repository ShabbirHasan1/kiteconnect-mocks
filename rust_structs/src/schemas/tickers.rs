use super::common::*;
use crate::utils::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::{DateTime, FixedOffset, TimeZone};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::sync::{Arc, Mutex};

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ltp {
    pub mode: TickerStreamingMode,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
}

impl From<&[u8]> for Ltp {
    fn from(mut buf: &[u8]) -> Self {
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
            !buf.has_remaining(),
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
    fn from(mut buf: &[u8]) -> Self {
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
            !buf.has_remaining(),
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
    fn from(mut buf: &[u8]) -> Self {
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
        assert!(!buf.has_remaining(), "After Parsing ChangeFromTickPacket<i32> QuoteIndex Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
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
    fn from(mut buf: &[u8]) -> Self {
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
        assert!(!buf.has_remaining(), "After Parsing ExchangeTimestamp<i32> FullIndex Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
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
    fn from(mut buf: &[u8]) -> Self {
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
        assert!(!buf.has_remaining(), "After Parsing ClosePrice<i32> Quote Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
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
    fn from(mut buf: &[u8]) -> Self {
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
                        120 - (idx * 12) - 10,
                        "Required Size of Full Buffer After Parsing\nMarketDepth {{\nbuy: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 120 - (idx * 12) - 10,
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
                        60 - (idx * 12) - 10,
                        "Required Size of Full Buffer After Parsing\nMarketDepth {{\nsell: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 60 - (idx * 12) - 10,
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
        assert!(!buf.has_remaining(), "After Parsing Depth<MarketDepth> Full Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
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
    fn from(mut buf: &[u8]) -> Self {
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
                        120 - (idx * 12) - 10,
                        "Required Size of CompactFull Buffer After Parsing\nMarketDepth {{\nbuy: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 120 - (idx * 12) - 10,
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
                        60 - (idx * 12) - 10,
                        "Required Size of CompactFull Buffer After Parsing\nMarketDepth {{\nsell: MarketDepthData {{\nquantity: <i32>,\nprice: <i32>,\norders: <i16>\n}},\n}} at index {} is {} Bytes, But Found {} Bytes",
                        idx, 60 - (idx * 12) - 10,
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
        assert!(!buf.has_remaining(), "After Parsing Depth<MarketDepth> CompactFull Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
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
    fn from(mut buf: &[u8]) -> Self {
        assert_eq!(
            buf.remaining(),
            492,
            "Required Size of ExtendedDepth Buffer is 492 Bytes, But Found {} Bytes",
            buf.remaining()
        );
        let token = buf.get_i32();
        let segment = get_segment(token);
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
        assert!(!buf.has_remaining(), "After Parsing Depth<MarketDepth> ExtendedDepth Buffer Should Not Have Any Bytes Remaining For Consumption, But Found {} Bytes", buf.remaining());
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

impl From<(Full, ExtendedDepth)> for FullExtendedDepth {
    fn from((full, extended_depth): (Full, ExtendedDepth)) -> Self {
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

    fn try_from(mut buf: &[u8]) -> Result<Self, Self::Error> {
        let num_threads = ::num_cpus::get() as i16;
        let tickers = buf.get_i16();
        // dbg!(tickers);
        if tickers < num_threads {
            let tickers_data = vec![0_u8; tickers as usize]
                .iter()
                .map(|_| {
                    let packet_length = buf.get_i16() as usize;
                    // dbg!(packet_length);
                    let tikers_data =
                        TickersData::try_from((packet_length, &buf.chunk()[..packet_length]))
                            .unwrap();
                    buf.advance(packet_length);
                    tikers_data
                })
                .collect::<Vec<TickersData>>();
            Ok(Self {
                tickers,
                tickers_data,
            })
        } else {
            let mut buf = Arc::new(Mutex::new(buf));
            let tickers_data = vec![0_u8; tickers as usize]
                .iter()
                .map(|_| {
                    let packet_length = buf.lock().unwrap().get_i16() as usize;
                    // dbg!(packet_length);
                    let packet_data = buf.lock().unwrap().chunk()[..packet_length].to_owned();
                    buf.lock().unwrap().advance(packet_length);
                    (packet_length, packet_data)
                })
                .par_bridge()
                .map(|(packet_length, packet_data)| {
                    TickersData::try_from((packet_length, &packet_data[..])).unwrap()
                })
                .collect::<Vec<TickersData>>();
            Ok(Self {
                tickers,
                tickers_data,
            })
        }
    }
}

impl TryFrom<(&[u8], &[u8])> for Tickers {
    type Error = &'static str;

    fn try_from((mut full_buf, mut extended_buf): (&[u8], &[u8])) -> Result<Self, Self::Error> {
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

impl TryFrom<(usize, &[u8])> for TickersData {
    type Error = &'static str;

    fn try_from((packet_length, mut buf): (usize, &[u8])) -> Result<Self, Self::Error> {
        let ticker_type = TickerType::try_from(packet_length as usize)?;
        let ticker_type_data = TickerTypeData::try_from((ticker_type, buf))?;
        Ok(Self {
            ticker_type,
            ticker_type_data,
        })
    }
}

impl TryFrom<(&[u8], &[u8])> for TickersData {
    type Error = &'static str;

    fn try_from((full_buf, extended_buf): (&[u8], &[u8])) -> Result<Self, Self::Error> {
        let ticker_type = TickerType::try_from(full_buf.remaining() + extended_buf.remaining())?;
        assert_eq!(
            ticker_type,
            TickerType::FullExtendedDepth,
            "The Sum of Full and FullExtendedDepth Bytes Should Be 676 For The TickerType To Be Of Type TickerType::FullExtendedDepth"
        );
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

impl TryFrom<TickerTypeData> for TickerType {
    type Error = &'static str;
    fn try_from(ticker_type_data: TickerTypeData) -> Result<Self, Self::Error> {
        match ticker_type_data {
            TickerTypeData::Ltp(_) => Ok(Self::Ltp),
            TickerTypeData::Ltpc(_) => Ok(Self::Ltpc),
            TickerTypeData::QuoteIndex(_) => Ok(Self::QuoteIndex),
            TickerTypeData::FullIndex(_) => Ok(Self::FullIndex),
            TickerTypeData::Quote(_) => Ok(Self::Quote),
            TickerTypeData::Full(_) => Ok(Self::Full),
            TickerTypeData::CompactFull(_) => Ok(Self::CompactFull),
            TickerTypeData::ExtendedDepth(_) => Ok(Self::ExtendedDepth),
            TickerTypeData::FullExtendedDepth(_) => Ok(Self::FullExtendedDepth),
            _ => Err("TickerTypeData Only Accepts TickerTypeData::Ltp, Ltpc, QuoteIndex, FullIndex, Quote, Full, CompactFull, ExtendedDepth, FullExtendedDepth !"),
        }
    }
}

impl TryFrom<(TickerType, &[u8])> for TickerTypeData {
    type Error = &'static str;
    fn try_from((ticker_type, buf): (TickerType, &[u8])) -> Result<Self, Self::Error> {
        match ticker_type {
            TickerType::Ltp => Ok(Self::from(Ltp::from(buf))),
            TickerType::Ltpc => Ok(Self::from(Ltpc::from(buf))),
            TickerType::QuoteIndex => Ok(Self::from(QuoteIndex::from(buf))),
            TickerType::FullIndex => Ok(Self::from(FullIndex::from(buf))),
            TickerType::Quote => Ok(Self::from(Quote::from(buf))),
            TickerType::Full => Ok(Self::from(Full::from(buf))),
            TickerType::CompactFull => Ok(Self::from(CompactFull::from(buf))),
            TickerType::ExtendedDepth => Ok(Self::from(ExtendedDepth::from(buf))),
            _ => Err("TickerTypeData Only Accepts TickerType::Ltp, Ltpc, QuoteIndex, FullIndex, Quote, Full, CompactFull, ExtendedDepth, FullExtendedDepth !"),
        }
    }
}

impl TryFrom<(&[u8], &[u8])> for TickerTypeData {
    type Error = &'static str;
    fn try_from((full_buf, extended_buf): (&[u8], &[u8])) -> Result<Self, Self::Error> {
        match TickerType::try_from(full_buf.remaining() + extended_buf.remaining())? {
            TickerType::FullExtendedDepth => Ok(Self::from((Full::from(full_buf), ExtendedDepth::from(extended_buf)))),
            _ => Err("TickerTypeData Only Accepts TickerType::Ltp, Ltpc, QuoteIndex, FullIndex, Quote, Full, CompactFull, ExtendedDepth, FullExtendedDepth !"),
        }
    }
}

impl From<(Full, ExtendedDepth)> for TickerTypeData {
    fn from((full, extended): (Full, ExtendedDepth)) -> Self {
        Self::FullExtendedDepth(FullExtendedDepth::from((full, extended)))
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

pub fn get_file(
    file_name: &str,
    extension: &str,
    file_options: (bool, bool, bool),
) -> Result<File, &'static str> {
    let (write, append, create) = file_options;
    let file = OpenOptions::new()
        .write(write)
        .append(append)
        .create(create)
        .open(format!("{}.{}", file_name, extension))
        .map_err(|_| "Failed to create file / Error opening file")?;
    Ok(file)
}
pub fn get_file_ts(file: File) -> Result<Arc<Mutex<File>>, &'static str> {
    Ok(Arc::new(Mutex::new(file)))
}

pub fn write_json_file(
    json_file: &File,
    tickers: &Tickers,
) -> Result<(), Box<dyn std::error::Error>> {
    serde_json::to_writer_pretty(json_file, tickers).unwrap();
    Ok(())
}
pub fn write_bin_file(
    mut bin_file: &mut File,
    tickers: &Tickers,
) -> Result<(), Box<dyn std::error::Error>> {
    ::bincode::serde::encode_into_std_write(
        tickers,
        &mut bin_file,
        bincode::config::standard()
            .with_big_endian()
            .with_variable_int_encoding()
            .write_fixed_array_length(),
    )
    .unwrap();
    Ok(())
}
pub fn print_tickers(tickers: &Tickers) {
    println!("{}", serde_json::to_string(&tickers).unwrap());
}
pub fn pretty_print_tickers(tickers: &Tickers) {
    println!("{}", serde_json::to_string_pretty(tickers).unwrap());
}
pub fn print_tickers_struct(tickers: &Tickers) {
    println!("{:#?}", tickers);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;
    use std::io::{BufRead, BufReader};
    #[test]
    fn test_tickers_raw() -> Result<(), Box<dyn std::error::Error>> {
        // let json_file = get_file(kite_tickers_raw, "json", (true, true, true))?;
        // let mut bin_file = get_file(kite_tickers_raw, "bin", (true, true, true))?;
        if let Ok(lines) = read_lines("./custom_mock_files/ws.packet") {
            for line in lines {
                if let Ok(line) = line {
                    let buf = bytes::Bytes::from(base64::decode(line)?);
                    // println!("{}", buf.len());
                    // println!("{:?}", &buf);
                    if buf.len() > 2 {
                        let tickers = Tickers::try_from(buf.as_ref())?;
                        // print_tickers_struct(&tickers);
                        pretty_print_tickers(&tickers);
                        // write_json_file(&json_file, &tickers)?;
                        // write_bin_file(&mut bin_file, &tickers)?;
                    }
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_tickers_raw_single() -> Result<(), Box<dyn std::error::Error>> {
        // let json_file = get_file("kite_tickers_raw_single", "json", (true, true, true))?;
        // let mut bin_file = get_file("kite_tickers_raw_single", "bin", (true, true, true))?;
        let line =
            "AbIALACJbgIAAAE7AAAAGQAAAWAAACLEAABf/wAAFDcAAAGQAAABqQAAATsAAAGkACwAtAsCAAAZRgAAAJYAABtzAGUrBgACSfAAAj44AAAa9AAAI2QAABJ6AAAZvgAsANFYAgAAVnIAAAAZAABkUwGU4O0AAvXQAAV2DAAAdUQAAJRwAABI+AAAaVoALADR1gIAAANrAAAAGQAAA+IAGRrCAAFFGQAAawgAAASwAAAFtAAAAtAAAATxACwA0jQCAAZ99AAAABkABsv2AAABRQAAHycAACAhAAby6AAHGGgABmnMAAa4tAAsAImVAgAAAAAAAACWAAAAAAAAAAAAAA4QAAAOEAAAAAAAAAAAAAAAAAAGOBcALADFqAIAA1CxAAAAZAAAAAAAAAAAAAAjWgAAI1oAAAAAAAAAAAAAAAAAAwN8ACwAxhgCAAAAWgAAAGQAAABiAAJSiAACh9oAAKLkAAAAbgAAAH0AAABaAAAAoAAsANENAgAAAAAAAAAyAAAAAAAAAAAAABeiAAAYnAAAAAAAAAAAAAAAAAAGXzYALADSCwIABP7jAAAAMgAFGGEAAAO2AAAxOAAAMQYABYbYAAWG2AAE5DoABR1HACwA1FUCAAAAAAAAAfQAAAAAAAAAAAAALq4AADWEAAAAAAAAAAAAAAAAAAJr5wAsALeWAgAAAAAAAAAyAAAAAAAAAAAAABw5AAAaRQAAAAAAAAAAAAAAAAAFkC4ALADRLQIAAAyKAAAAGQAADiIAF/BrAADTbQAAhX8AABMpAAAWbAAACfYAABGoACwApZMCAAC/5QAAADIAALoEAAOp5AABOXoAAMmQAADDHgAA33oAAJ9CAADLGwAsAMYsAgAAAFUAAAV4AAAAWgADlTAAA4CuAAF3yAAAAIwAAACMAAAASwAAAIIALADSHAIABbkoAAAAGQAAAAAAAAAAAAASKgAAFRgAAAAAAAAAAAAAAAAAAtvgACwAxZQCAAAAhwAAADIAAACPAAV2ogAD0MIAAKeUAAAAtAAAALkAAAB9AAAApQAsAMYgAgAAAF8AAAPoAAAAagAAUwIAA4l4AABaCgAAALQAAAC0AAAAWgAAAL4ALADRLgIAAAAAAAACCAAAAAAAAAAAAAAaEwAAGfoAAAAAAAAAAAAAAAAABS7CACwA0TcCAANACAAAABkAAuVrAAAmxQAAfkUAACw9AAKwVwADUycAApc5AASvgwAsANGCAgAAbXQAAACvAABpggCAL6AAARqeAAGN8wAAX6oAAImyAABEjgAAZgMALADRogIAABueAAAAZAAAG4EATHcZAADIGQAA2IEAAA6wAAAoVQAADrAAABrHACwAsQ8CAABmbAAAAMgAAGwvACE9kgACl94AAYPkAABmsgAAhX8AAFHHAABlIgAsAL/9AgAAAMgAAAAyAAAA1wAQCOIAAVm0AAEslgAAASIAAAEiAAAAlgAAAQ4ALADRdwIAAKGfAAAAGQAAsRsA+O1LAALBeAAC40IAANrFAAD96AAAjKUAALxNACwAvZsCAADs4AAAADIAAO6EAAAOQgAAtwIAAFB4AADmcwABDRAAANLwAAD7GAAsAL/zAgAByXsAAAAyAAHC5gAAAV4AAFJsAABOhAABz0MAAc9DAAGzGQABrJMALACPwAIAAAAAAAAPoAAAAAAAAAAAAAATCwAAEwsAAAAAAAAAAAAAAAAABhruACwApZgCAACg1wAAADIAAJujAC4YMgAEcLgAATbwAACivAAAvm4AAIL6AACrqQAsANFFAgACXkAAAABkAAIEMgAABdwAAGlGAAA0DQACEfEAAmGwAAHYFwADdq4ALADRsQIAAm9/AAAAGQAComsAAA0WAACGLgAAO8QAAt5gAALkGQACIpoAAnwEACwAj7MCAAAAAAAAABkAAAAAAAAAAAAAEB0AABAdAAAAAAAAAAAAAAAAAAX30AAsAL/iAgAAAeUAAAAyAAACUgBr6AYABMg4AAHDLAAAA2sAAAO2AAAB0QAAA+gALADGJwIABRrCAAAAZAAFIwAAAADIAAAchAAAHIQABSs+AAUrPgAFGsIAAiyLACwA0fACAARKSAAAADIABGcAAAAAfQAAGqkAAByEAASXRgAEl0YABD/kAASPEgAsANISAgAFWGEAAAAZAAVYYQAAABkAAA4QAAAOEAAFWGEABVhhAAVYYQAEF0QALACJYwIAAAAAAAAAMgAAAAAAAAAAAAAOEAAADhAAAAAAAAAAAAAAAAAABbBZACwAlXUCAAAEqwAAADIAAATQAOrtNgAGe/YAA0AIAAAD6AAABl4AAAMvAAAE2AAsAL/XAgAA6NoAAAAyAADwagADem4AAMR8AABt9gAA5N4AARQxAADJLAAA4qkALACIzwIAB7OGAAAAGQAAAAAAAAAAAAAUtAAAFLQAAAAAAAAAAAAAAAAAB5SvACwAiWsCAAAAAAAAABkAAAAAAAAAAAAAFLQAABS0AAAAAAAAAAAAAAAAAAXzwAAsAKWWAgAArm8AAAAyAACq3QAAp5QAAQMuAAC4YAAAttUAAMuOAACS/gAAvbUALACY5gIAAMilAAAAMgAA0A8AEi/cAADLIAAAwiQAAMCUAADweAAAp8YAAMFrACwA0RQCAAAGlQAAAEsAAAd/AEUr6QAA7pMAAK27AAALHQAACx0AAAV9AAAJZQAsANIdAgAAAYsAAAAZAAABrAAQhw4AAX9/AABeCwAAAV4AAAINAAABXgAAAiEALACvYQIAAaJwAAAAMgABnyMAAACWAAA4pAAANIoAAZNmAAGnkwABk2YAAltNACwApaACAACDnwAABH4AAH+aAC0w8gADU5AAART0AACH6wAAn0cAAGjxAACOXQAsAJa+AgAAQfAAAAAyAABIYgHQXF4AAsLWAAVsegAARloAAFkGAAAy0gAAQfAALADFMQIAAo5RAAAAZAAClPAAACBsAABMkAAARewAAoo8AAK4nwACcWQAAoDSACwAxioCAAAAWgAAAGQAAABZAAFHvAABreIAAJ7KAAAAlgAAAJYAAABLAAAAjAAsANErAgAAC2gAAAAZAAAM2wAXcH0AAOouAABklgAAEG0AABQjAAAJJAAAD6UALAC/7gIAAAETAAAAMgAAAUcAMCUyAAKMJgABbswAAAGpAAABxwAAARMAAAHlACwAxTUCAAKhGwAAADIAAAAAAAAAAAAAJ0IAAB+kAAAAAAAAAAAAAAAAAAIDoAAsANE6AgAAF4kAAAAZAAAbkAAhH+IAALE/AABvhgAAIygAACpdAAASwAAAH8wALADRgQIAAOwdAAAAGQAA/ykAWOKwAAHzHwABJwUAATgXAAFd4gAA0aEAAQh0ACwA0i0CAAABcgAAABkAAAGJAAILVwAAwuwAAB1MAAABqQAAAg0AAAFZAAAB6gAsANSgAgABJiQAAABkAAEc+AAAfDgAAKCMAABUxAABFdAAAUcmAAECrAABPWwAHAAD+AkAMerAADIZQQAxRX8AMW2xADHFDgAAJbIALADRQQIAAnM6AAAAMgACOw8AAAF3AABoTAAAMksAAi1YAAJzOgACHV4AAw2kACwAxhkCAATQCAAAAGQABMMuAAAAyAAAHIQAAByEAAS2VAAE0AgABLZUAAHWDwAsANE8AgAAGfAAAAAZAAAesAAgDwgAAJJKAABmPwAAKjAAAC92AAAU8AAAI9IALADR+QIAAAH0AAAAGQAAAiIABZieAADSQQAATugAAAIhAAACvAAAAcwAAALVACwA0jcCAAABXgAAADIAAAGFAADWvwAAX80AABJDAAABwgAAAcwAAAFeAAAB1gAsAL1/AgAAAeUAAADIAAAB6ABTRfgACodQAAJOPAAAATYAAAKAAAABNgAAAAoALACWuwIAAGjsAAABXgAAZRcBLlc4AATHPgAC0B4AAGXMAACCFAAAUZ8AAHORACwA0c0CAAM0UAAAABkAA2IMAAABdwAAPk4AACpiAAPCHAADwiEAAyGVAANJzAAsALEQAgAAKo8AAABkAAAq8gORwhIACElKAAttyAAALKYAADu6AAAfkAAANFgALADGFAIAAABkAAAAMgAAAGAAARQsAAOeXgAAnjQAAACHAAAAhwAAAF8AAACgACwA0TgCAAAVGAAAABkAABhbABdkYQAAnekAAGDgAAAc3gAAJhYAABDMAAAb7gAsANFMAgAB5FsAAAAZAAG49AAASoMAANfSAABBCgABswAAAhgnAAF3yAABxoMALACIxAIAAAFKAAAAGQAAAW8AADLIAABULgAAEioAAAGaAAABqQAAAUoAAAHRACwA1KECAAANVwAAADIAAA5fAMwsjgABuDwAAkJSAAAOpgAAE3kAAAmDAAANcAAsAMUwAgAAAJsAAABkAAAAsgCe6HgATX7eAAfaWgAAALQAAADrAAAAhwAAANIALACPtAIAAAEEAAAAGQAAAR4AECC2AANCkgACbl0AAAExAAABfAAAAP8AAAF8ACwA0WACAAEaIQAAABkAAPvWAD9y3wABxpcAAISeAADfLwABQqgAAMYMAAEGgAAsAL/jAgABS0AAAAAyAAFMGgAABUYAAGe2AABKBgABXHUAAVx1AAE3nwABPgwALACI9gIAAAETAAAAGQAAASoAABGtAADNRgAADvEAAAExAAABNgAAAPAAAAGpACwA1JsCAAAJHwAAADIAAAmyAC423AAA/bYAAUoUAAALSgAADOQAAAZeAAAJJAAsANIuAgAGdFgAAABLAAZ0WAAAAdsAABLA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VeUALADdnQIAMfv/AAAAGQAx2T4ALlYcAAHuugACe1oAMZdBADIylgAxVtwAMcuPACwA0UICAAAgewAAABkAACYGACmzfQAAujsAAJ5/AAAw6AAAO7UAABoxAAApuAAsANFcAgABMu8AAAAyAAEWVQAYmK4AAdYeAACDDgAA/coAAV6vAADcvgABHjEALADRvwIAAs8BAAAAGQAC9CgAADJkAACDJwAAOuMAAxNdAANS1wACjlEAAuluACwAvXgCAAIlYAAAADIAAkU/AAABkAAANuIAACjSAAJMnQACW44AAiVgAAJTRgAsALEZAgAAIygAAAAyAAAjbQDRD+wAAqReAAMEEgAAKNwAADK5AAAaGAAALHkALADSLAIABhq8AAAAGQAGGrwAAABLAAAUtAAAFLQABhq8AAYavAAGGrwABklgACwBQjsCAAABLAAAABkAAAFPAAHUKgABDSQAACU1AAABXgAAAeAAAAEnAAABmgAsAMXNAgAAAHgAAAAyAAAAfQABbaAAAZImAACQugAAAKUAAAC+AAAAcwAAAKUALADF4gIAAABpAAAAyAAAAHAAAk48AAFcPgAArQwAAACbAAAAmwAAAGkAAACbACwAt5ECAAADwAAAAGQAAAQeAAtfhgAA+3cAADRxAAABlQAABjYAAAGVAAAAQQAsAMX0AgAEJ1cAAAAyAAAAAAAAAAAAACNaAAAchAAAAAAAAAAAAAAAAAADjDQALAC3jwIAAANwAAAAyAAAA9YADBTGAAECmAAAJmEAAAHWAAAExAAAAdYAAAAyACwA0VoCAABgHQAAABkAAHDzAIkIwwABVZoAApU7AACI2wAAo8oAAFIIAABzVQAsANF6AgAApBAAAAAZAACewAEcCE4AAgehAAM0zQAAkncAAMWyAABq0QAAmn4ALACWxAIAAFwhAAAAMgAAYGEBS6XCAAQtiAACnPIAAGGoAAB5GAAASNoAAFtAACwAxTQCAAAAlgAAADIAAACoAAbVBgADhJYAARP6AAAA+gAAAPoAAACRAAAAyAAsANH9AgAEzJ0AAAAZAATQCwAAAMgAABc+AAAUtAAE/iAABP4gAAS/LQAEKHQALAFCPwIAAAE2AAAAGQAAAUwAAE9lAABnhAAAERcAAAGfAAABqQAAATEAAAGfACwA1FYCAAAHAwAAADIAAAdXAChV+gAAdsAAARVYAAAH3wAACcQAAATOAAAHKwAsAMYfAgAAAAAAAAj8AAAAAAAAAAAAABzoAAAdTAAAAAAAAAAAAAAAAAAB8gwALADRPgIAAB0LAAAAGQAAIcsAtZ+0AAJSiAAB4oUAACyIAAA2bwAAF3AAACadACwA0ZECAAA5xgAAABkAADajASOsKgADN7sAA2h2AAAqMAAATUkAACLOAAA2JAAsANGqAgAAEqIAAAAZAAATGwA2qtMAALbQAAC11gAADhAAABxwAAAMEgAAEo4ALADRlgIAADBmAAAAGQAALl4AY0D8AAEEpQABWpUAACvAAABCNgAAHTMAAC5KACwA0eACAAACewAAABkAAALiABENPAABItIAAEJPAAAD2QAAA+MAAAJYAAAD4wAsAMWvAgAAAH0AAABkAAAAfwABQkQAAiVqAACg8AAAAKUAAAClAAAAaQAAAKAALADGIQIAAAAAAAAAGQAAAAAAAAAAAAAjWgAAHIQAAAAAAAAAAAAAAAAAAgBOACwA0U8CAAA+igAAABkAAEfbAEcb6AAA0LEAAV4yAABXOgAAbi0AADPlAABM4AAsANGLAgABGbgAAAAZAAE1GgAFydYAAM+FAAB4NwABWtsAAZT7AAD8/QABPDYALADF6gIAAABpAAAAMgAAAGwAA4XCAAEjNgAAt5gAAACgAAAAoAAAAGQAAAClACwA0g8CAAUcmAAAADIABS1jAAABLAAAEvIAABWuAAV2tgAFjAUABRyYAAVw5AAsANFUAgABjqcAAAAZAAFrpwABn24AANDjAABEEQABUgwAAbqoAAEnaQABbesALADFsAIAA2Q0AAAAZAAAAAAAAAAAAAAjWgAAI1oAAAAAAAAAAAAAAAAAAgSkACwA0TsCAAAAAAAAAFAAAAAAAAAAAAAASNoAACigAAAAAAAAAAAAAAAAAARyBwAsANFTAgAARYgAAAAyAABQrwBhL/sAASCsAAFaGAAAXfwAAHo6AAA6KgAAVTwALACIvwIAB2lsAAAAGQAAAAAAAAAAAAAg6QAAHTMAAAAAAAAAAAAAAAAAB2lsACwBQkMCAAABJwAAABkAAAFJAAB6jwAAanIAABKOAAABgQAAAZAAAAEnAAABbQAsAMWVAgADFwQAAAAyAAAAAAAAAAAAACYWAAAe3AAAAAAAAAAAAAAAAAADFwQALADGMwIAAAAAAAAAZAAAAAAAAAAAAAAchAAAHIQAAAAAAAAAAAAAAAAAAlmkACwA0Q4CAAAGBAAAABkAAAbWAA/4rAAARuYAAE/JAAAJ9gAACfsAAAUUAAAIkwAsAUI5AgAAAUUAAAAZAAABWgAAHvUAAFxiAAAUBQAAAW0AAAGQAAABNgAAAa4ALAFCRQIAAAEdAAAAGQAAAToABXFcAAHUpwAAnWwAAAE2AAAB5QAAARgAAAGQACwAxf4CAAAAaQAAADIAAABwAAA+5AABSw4AAKm6AAAAoAAAAKAAAABLAAAAoAAsAIkaAgAAAAAAAAB9AAAAAAAAAAAAAA4QAAAOEAAAAAAAAAAAAAAAAAAFjv0ALACvYgIAAAU8AAAAMgAABYUANdhgAACQ7AABL1IAAAScAAAHgAAAA6IAAAV9ACwAv9kCAAD6AAAAADIAAPz2AAATugAAquYAAFWMAAESTAABIqAAAOG5AAD1WgAsANFVAgAATRwAAAAZAABZzwBfWr4AASDeAAGcAwAAcUMAAIb7AABBRgAAXdkALACJAwIAAAAAAAAAMgAAAAAAAAAAAAAQHQAAEB0AAAAAAAAAAAAAAAAABdvJACwAxWYCAAAAlgAAADIAAACiAAAbWAABLu4AAH+8AAAAyAAAAMgAAACHAAAAwwAsANF8AgAAkGUAAAAZAACLaQCWvb8AAaZdAAG/dgAAbzYAALD0AABcowAAiHcALADSDAIAAAGpAAAAGQAAAdoABDmLAAC5cwAAH1kAAAGkAAACYgAAAXcAAAJxACwAv/YCAAAA4QAAAJYAAAEEAUQU2AAt31wABogSAAABNgAAAVQAAADhAAABWQAsAMYjAgAFAQkAAAAyAAURowAAAPoAAByEAAAchAAFBKYABSbtAAUBCQAEzngALADRqQIAAgl8AAAAGQACOw4AAtotAADotwAAWm4AAmSUAAKd5wAB3zgAAi+wACwAj7ICAAABCQAAABkAAAEhAAEuigAAihYAACbeAAABhgAAAYYAAAEEAAABhgAsANSdAgAACm4AAAHCAAALKgF+874AA7YAAAO2yAAAC0UAAA6/AAAHUwAACpYALADUnwIAAAvMAAABXgAADLIAMeqEAAH1LAABIaYAAAzkAAAQzAAACGEAAAv5ACwAv+sCAAABRQAAAJYAAAF2AVv17gAKB8YABaAKAAAB2wAAAiEAAAE7AAACOgAsAMWYAgAAAIwAAAEsAAAAhQABcMAAAWRAAACD1gAAALQAAAC0AAAAfQAAALkALADR3wIAA+J0AAAAGQAEUcIAAAYnAAAawgAAHLYABHxwAAR/iwAD4nQABAdAACwApaMCAAA0ZwAAADIAADkkAMJM0gACByQAAvi+AAAvTgAAR5oAACfYAAAzswAsAJjMAgAAmFgAAABkAACdPAACC3AAAQDWAADBwAAAnA4AAL1WAAB95gAAlgUALADE8AIAAACqAAAAMgAAAL8AQN8IAAqm9AACzTAAAADDAAAA5gAAAKUAAADrACwAxPcCAAAArwAAADIAAAC+AAXYSgAA2ZQAAL82AAAAzQAAAPoAAAClAAAA5gAsAMYeAgAAAF8AAAAyAAAAZQAQTEQAFwNoAALYtgAAAF8AAACHAAAAVQAAAIwALADSDgIAAAGfAAABLAAAAcUAAy3FAADl+wAAJAkAAAI6AAACcQAAAYsAAAJYACwAiQ8CAApx6AAAABkAAAAAAAAAAAAAERcAABEwAAAAAAAAAAAAAAAAAAd+6AAsAMVjAgAAAJEAAABkAAAAnwAIHlIAA2YeAAECZgAAAL4AAAD/AAAAggAAALkALADRLAIAAAAAAAACCAAAAAAAAAAAAAAZfQAAGWQAAAAAAAAAAAAAAAAABU9qACwA0TQCAANbYAAAABkAA0X0AAAEZQAAdRcAAB7DAALUVgADXQQAAtRRAATOzQAsANFhAgAAds8AAAAZAACIvAEDKCQAAquYAAOCVwAAo44AAMPNAABmCAAAi/EALADRmgIAAChuAAAAGQAAJuYAcJftAAFj3AABNE0AACS4AAA4SgAAGHQAACbZACwAv9sCAAEM4wAAAGQAARYhAAhdBAAJk8IAANWsAAELlAABOLwAAOpgAAEEvgAsANGMAgAAUH0AAAAZAABM+QBxsEwAARo6AAFXwAAAR4sAAGkAAAAxiAAASyMALAFCPAIACOusAAAAMgAAAAAAAAAAAAAVrgAAFa4AAAAAAAAAAAAAAAAACMQkACwA1JwCAAFFyAAAAGQAAUE/AAEa0AAAp/gAAFJsAAFSNAABZ6sAASJpAAFSNAAsAMWMAgAAAJEAAAAyAAAAmgAFpIgAAtScAAMxMAAAAL4AAADrAAAAggAAAK8ALADGGgIAAABkAAAAMgAAAGQAAq9OAAFxVgAASNoAAACRAAAAkQAAAFUAAAC0ACwAt5QCAAAAAAAAAOEAAAAAAAAAAAAADwoAABAEAAAAAAAAAAAAAAAAAAW3GwAsANEqAgAAAAAAAAAoAAAAAAAAAAAAABpeAAAXiQAAAAAAAAAAAAAAAAAFcGIALADSKwIAAAF8AAAAGQAAAZYAAUe8AADOpAAAKG4AAAHlAAAB9AAAAWMAAAHqACwAiPUCAAAAAAAAASwAAAAAAAAAAAAACXkAAAl5AAAAAAAAAAAAAAAAAAWW3AAsANSeAgABJj0AAAAyAAEp7gAABzoAAGzKAABsNAABJPMAAT4WAAEZ0QAB994ALACllAIAAByJAAABLAAAHzwBdinwAANmUAAEtH4AACLEAAAn+wAAFQ4AAByEACwAwXcCAAAAuQAAADIAAADNAHPL8gAG/jwAAhOkAAAA+gAAAQQAAAC0AAABCQAsANG1AgACjc8AAACvAAKREQAABV8AAIu/AAA7FQACvyAAAvcVAAJvIAACpBgALADR+wIAAAHRAAAAGQAAAgUABSADAADXCgAAJ1sAAAIIAAACjwAAAbgAAAK3ACwAiLwCAAABVAAAABkAAAF7AABdwAAAXdkAABCzAAAB4AAAAeUAAAFPAAAB1gAsAUJIAgAAAAAAAAGXAAAAAAAAAAAAABS0AAAQHQAAAAAAAAAAAAAAAAAGLQQALAC9igIAAf3EAAAAMgACAV8AAAbWAAA4DgAAM5AAAfdIAAIi4AAB5ZYAAhjRACwAv94CAAACTgAAADIAAALWAWU2GAAG4oAAA8sYAAAD3gAABLUAAAIrAAAE3QAsALeEAgAGR8sAAAAZAAZhQAAAADIAABPsAAAU5gAGerYABnq2AAZHywAGK/sALADSCgIAAAG4AAAAGQAAAekAKrzSAAHTrQAAsMIAAAFeAAACbAAAAV4AAAKUACwAmNwCAAASVwAAAGQAABMkApAOZAAGIuYAB/uOAAARlAAAHUcAAA3FAAAZoAAsAL/oAgAAAWMAAAAyAAABpwBEUm4AAczwAAJfngAAAkkAAAKKAAABWQAAAo8ALADGNQIABXtwAAAAMgAFgeIAAAGQAAAc6AAAHOgABXiRAAWXkAAFeJEAAmkSACwA0icCAAABgQAAABkAAAGdAAJYyAAAx4MAABr0AAAB4AAAAj8AAAFoAAACCAAsAImPAgAAATsAAAAZAAABXwAAGWQAAGO1AAAOvwAAAZoAAAGzAAABOwAAAccALADRbQIAAJHrAAAAGQAAo4AB5ZJyAAMa0wAF5MoAAMEgAADpiQAAfnIAAKqMACwA0g0CAAUS1AAAABkABRLUAAAAGQAAEd8AABS0AAUS1AAFEtQABRLUAATKkAAsAL2NAgAAAt8AAABkAAAC3wDiq1gAFkkUAAPUeAAAAUoAAAOdAAABSgAAAB4ALADRbAIAAObXAAAAGQAA1DwBSUDFAANEbQABuq0AAL+VAAENPQAAnbIAANSUACwA0X8CAADXGQAAABkAAOZPACOdrQABiY4AAMz7AAEB+AABQ+gAAL6vAAD1mwAsAUJAAgAAAAAAAAV4AAAAAAAAAAAAABS0AAAUtAAAAAAAAAAAAAAAAAAFo50ALADFZwIAAtukAAAAMgAAAAAAAAAAAAAlsgAAI/AAAAAAAAAAAAAAAAAAAt+lACwAt5UCAAAEAQAAADIAAAR4AAlAdQABKOAAADvdAAACEgAAB/gAAAISAAAAVQAsANHJAgAABf8AAABkAAAGrgAkQrcAAW9iAACV5wAABqQAAAoZAAAE8QAAB0QALADF0gIAAAB4AAADIAAAAHYAARsCAAHgeAAApjYAAACbAAAAoAAAAGQAAACgACwA0hsCAAABlQAAABkAAAGxAAHAuwAA5TMAACHjAAABnwAAAjoAAAF3AAACNQAsAKWXAgAAIIAAAAAyAAAjOgCTO/4AAsZaAAJbhAAAIygAAC0eAAAX/AAAIE4ALADRJQIAAAiJAAAAGQAACcIAFPR0AADewQAAYfMAAAxOAAAOxAAABwgAAAxJACwA0bICAAAMgAAAABkAAA0HADWA+QAA/uIAAJ8uAAAGrgAAE+cAAAauAAANQwAsAJbCAgAAUqgAAAAyAABV1QDGXmwAAsfqAAH70AAAVfUAAG1vAABAvwAAUhIALACY6QIAAAi7AAAAMgAACXYArrpGAAJKuAACp+IAAAvvAAAPoAAABxIAAA5RACwAxOoCAAIiGAAAADIAAhtQAAAJkgAAUQ4AAEJoAAIsLAACQ/YAAgxRAAIOEwAsANHRAgADiAEAAAAyAAOwBgAAPIwAAD07AAA3LQAEA6MABBm1AANTCQADodgALAC/8gIAAADrAAABXgAAARYANnRuAAH99gABf8oAAAFyAAAB6gAAAOEAAAFyACwA0TkCAAAAAAAAACgAAAAAAAAAAAAAR+AAACd0AAAAAAAAAAAAAAAAAASQmAAsANGIAgAAXiQAAAAZAABakACUXVkAAU4uAAHbMgAAUHgAAHh9AAA6TQAAV3EALACY4wIAAA6IAAAAlgAAD14AuAYoAAK5dgACyBwAABNMAAAYAQAACx0AABVPACwAv/sCAAAAwwAAAfQAAADkAIzV6AAJ17oAA4a8AAAAbgAAAScAAABuAAABIgAsAMWlAgADUGYAAACWAANWOwAABnIAAC0eAAAlgAADVVIAA3V4AAM3OQADQKMALADF9wIAAAAAAAAAGQAAAAAAAAAAAAAjWgAAHIQAAAAAAAAAAAAAAAAAAZNrACwA0Y4CAABEXAAAABkAAEF0AGeQ5AABH/0AASjHAAA16AAAWn0AACmpAABAFQAsANRSAgAABhgAAAD6AAAGVQCmi4wAAT6OAAHT+AAABW4AAAiEAAAEKQAABjYALADUmgIAAVPYAAAAMgABUHoAAAHCAAA4pAAAMAwAAVQyAAFfkAABQKUAAxV0ACwAxfYCAAAAaQAAADIAAABrAAI6tAABHrgAAEQqAAAAjAAAAIwAAABaAAAAqgAsALeHAgAAAAAAAADwAAAAAAAAAAAAAA8KAAAQBAAAAAAAAAAAAAAAAAAGBQQALAC/7AIAAYI7AAAAMgABilUAAGKiAACOMAAAViIAAYMmAAGmCAABXrQAAXROACwAxS4CAAAAoAAAAMgAAAC4AAKkLAAA41gAALVAAAAArwAAAOYAAACgAAAA3AAsAMYtAgAAAAAAAADIAAAAAAAAAAAAABy2AAAchAAAAAAAAAAAAAAAAAACSl4ALADRbgIAAM7qAAAAGQAAwakA5G6dAAKWmQABwvoAALDCAAD0JAAAi5wAAL+fACwAiWUCAAABQAAAABkAAAFZAAWBqwABGzQAAGWpAAABgQAAAeAAAAExAAABqQAsAIlqAgAAAUAAAACWAAABaAAAMgAAAGhMAAAUNwAAAZoAAAGaAAABNgAAAaQALADFMwIAAo2dAAAAZAAAAAAAAAAAAAAlTgAAI1oAAAAAAAAAAAAAAAAAAd+1ACwAxiICAAAAXwAAADIAAABmAATsvgAC/8YAAMAwAAAAjAAAAIwAAABQAAAAjAAsANF7AgAAw+sAAAAyAADQcgB8zq8AAWGdAAGUlwAA+x0AASvEAACshQAA30MALACIzgIAAAFKAAAAGQAAAW0AADvdAABiiQAAFAUAAAHMAAABzAAAAUoAAAHbACwAiPQCAAABJwAAABkAAAE0AACbkQAAgl8AABc+AAABBAAAAXwAAAEEAAAEYAAsANEkAgAAAAAAAAImAAAAAAAAAAAAABl9AAAWjwAAAAAAAAAAAAAAAAAF1RYALADRJwIAAAlHAAAAMgAACq8AF/k1AADHgwAAWN4AAA3yAAAQdwAAB6gAAA0MACwA0XkCAACyYQAAAXcAAL7wANaImwAB2HYAAn01AADnxwABFBMAAJxAAADMxAAsANHMAgAABQAAAAAZAAAFoQAlgiYAAVJhAACHDwAABQUAAAinAAAETAAABoEALADR2AIAAAMCAAAAGQAAA30AGtd8AAEwsAAAZK8AAAKAAAAE9gAAAoAAAAR+ACwAiL4CAAABTwAAABkAAAFoAAUQrgAAzMkAACnlAAABgQAAAcIAAAFAAAABzAAsAL/YAgAABQoAAAAyAAAGBAByDqIAAlvoAAGOPgAAB2IAAAnYAAAD8gAACXQALADE7wIAAjKAAAAAlgACMVQAAAEsAABLyAAAR3wAAjAoAAIygAACMCgAAcrKACwA0V8CAABq0QAAABkAAH08AMNrPwABnXoAAxRhAACS6gAAs2oAAFtyAAB+aAAsANHsAgAAAiEAAABLAAACbgAJUQ8AAOZ4AABAKQAAAo8AAAMvAAACAwAAA0MALACxGAIAAHHKAAAAMgAAeHkAuvrMAAOjDgACByQAAHgPAACS0QAAW14AAHB7ACwAv/cCAAHTTgAAAGQAAdsSAAA6/AAAaBoAAEuWAAHLxAAB8hsAAagLAAHArAAsAUJBAgAAATEAAAAZAAABSgAAWMUAAGrvAAAUtAAAAdEAAAHRAAABLAAAAa4ALACI9wIAAAAAAAAASwAAAAAAAAAAAAAQHQAAEB0AAAAAAAAAAAAAAAAABbk3ACwAv+8CAAGY2QAAADIAAZOFAAAttAAAdioAAEz0AAGnYQABu1wAAYSsAAGCHQAsANGcAgAAIVcAAAAZAAAg+ABmuu0AAR2lAAFR/QAAHCAAAC/BAAAUaQAAICEALADR6QIABB6wAAAAGQAEel8AAAOdAAAX7QAAGDgABKwOAAS9XAAD96AABDbAACwAmOQCAAC6qQAAADIAAL8MAAGoagAA24gAAKRCAAC+kQAA3uQAAJnKAACx/QAsAL/wAgAAAQkAAABkAAABMAC95LIAB+peAAS13AAAAYEAAAGQAAABBAAAAakALADRrwIAAkMpAAAAGQACaLkAAAlHAACGqwAAOxUAArDtAAK7xAACBwsAAlkdACwAxiQCAAAAWgAAADIAAABbAAJzigABu44AAJZkAAAAlgAAAJYAAABVAAAAlgAsALeQAgAAAAAAAAEsAAAAAAAAAAAAAA8KAAAQBAAAAAAAAAAAAAAAAAAF3g0ALADdngIAF2GqAAABLAAXWp4AvNxwAAmuIAAIZWoAF1qOABeKbQAXNg0AF2uHACwA0S8CAAAN2QAAABkAAA/hABXxFwAAuMQAAHKNAAAUXwAAGN0AAAsOAAAS/AAsANExAgAAD18AAAAZAAARiwAVhkEAANkwAABcrQAAFhcAABueAAAMKwAAFTEALADSHgIABftAAAAAGQAF2LsAAAM5AAAghQAAI6UABhqAAAY12AAFtM0ABetGACwA0jMCAAABbQAAABkAAAGJAA9khwABXD4AAIy5AAABRQAAAeAAAAFFAAAB6gAsANHXAgADlfgAAAAyAAOX5QAABDMAACRtAAAnEAAEOwwABDsMAAOCcAADzuwALADGMgIAAABVAAAAMgAAAFkAAcP0AANUvAAAwMYAAACHAAAAhwAAAFAAAACRACwA0QwCAAAFhwAAAEsAAAZZABbLpQAAftsAAF+CAAAH0AAACcQAAASwAAAHtwAsALEbAgAAHMUAAACWAAAdfAKuHR4ABbzyAAfzvgAAIfcAACrBAAAVTwAAJVgALAC/1gIAAAafAAAAMgAAB4kB0D5KAAQVHgAFIUgAAAlbAAAMbAAABXgAAAuVACwA0XgCAAC4qwAAABkAALGtARptdwABzrIAAl8hAACcJwAA3K8AAHqtAACtcAAsAJpBAgABAFQAAAAyAAD9iwAAZJYAALlaAABjnAABDJgAASI8AADeOgABDlAALAC/2gIAAAPoAAAAMgAABLkCqFUKAAuWaAAPU3AAAAaaAAAHwQAAA2sAAAeeACwAv/kCAAH5IwAAADIAAfqLAAAAZAAASXAAAEPGAAH78wAB+/MAAfkjAAHZEQAsAMU4AgACtJ4AAAAyAAAAAAAAAAAAACWyAAAlTgAAAAAAAAAAAAAAAAACMo8ALADRCwIABSDfAAAAGQAFISIAAAj8AAAfWQAAGUsABUBbAAVHHQAFHJgABoJZACwA0jgCAAAAAAAAAJYAAAAAAAAAAAAAFLQAABS0AAAAAAAAAAAAAAAAAAYJGQAsAIkQAgAAARMAAAAZAAABJAAA1ikAARtmAAAa9AAAATEAAAFPAAABDgAAATYALADFvwIAAABzAAABLAAAAH8AAHp2AAIqTAAAngIAAACvAAAArwAAAG4AAACgACwA0TMCAAARDQAAABkAABPLAFkqEwABISkAAQxcAAAdtQAAIHEAAA2JAAAXjgAsAJa8AgAAOtkAAABkAABANwN8V9gABQpuAAu3OAAAPoAAAFAAAAAtBQAAOoQALADGNAIAAABQAAAFFAAAAFgAJvJQACLEZAAJZH4AAAB9AAAAfQAAAEsAAACHACwApZsCAAApMQAAAGQAACxEAINpaAABzL4AAmpcAAArFgAAOQgAAB8JAAAofQAsAL/nAgABWf8AAAAyAAFfzQAAvzYAAHeIAABbmgABb8EAAYPpAAE3OwABTwAALADF5QIABCH4AAAAMgAEKCwAAAD6AAAiYAAAJYAABDPSAAQz0gAEH6AABAOAACwA0aECAAHJXQAAABkAAfXKAABNDQAAok4AAD7LAAIY0QACV0IAAZ5HAAHuAQAsANH0AgAEp2gAAAAZAASnaAAAABkAABftAAAYBgAEp2gABKdoAASnaAAEhjkALADRzwIAA6mAAAAAGQAD4DoAAAGQAAA5UwAAKNIAA+kTAAPpGAADqYAAA4z3ACwAmOUCAAALQAAAAooAAAwyApxCnAAFqpYACngUAAAH0AAAE28AAAfLAAARWAAsAMXYAgAAAHgAAABkAAAAdQAAXDAAAY9qAACY7gAAAKUAAAClAAAAXwAAAKAALADRwAIAAAcNAAAAGQAAB9UAKetyAAFzGAAAm5EAAAdsAAAL7wAABGoAAAh1ACwApaICAAB17gAAADIAAHIQABnoOgACgG4AAXkmAAB89gAAkG8AAFzGAACAfwAsAMWNAgAC/k8AAAAyAAL/xwAAAcIAACcQAAAgbAAC+isAAw/3AAL6KwAC+nYALADRsAIAAA83AAAAGQAAD74AQwrLAAEApAAAvSkAAA7YAAAXuwAACh4AAA/mACwA0b0CAAK8oAAAABkAAtv0AAABwgABmcQAADtHAAMksAADJLAAApXRAALHlQAsAMXAAgADjigAAAAyAAAAAAAAAAAAACNaAAAjWgAAAAAAAAAAAAAAAAADjigALAC3hQIAAAMRAAAAlgAAA38AQO35AAKKSwAA5ngAAAFPAAAHywAAAU8AAAAjACwA0UQCAAAkJwAAABkAACmmAC5m6AAAt7EAAJpMAABDigAAQ4oAAB1RAAAtzQAsAUJHAgAAAScAAAAZAAABMwAAD+sAALvkAAAQ5QAAAScAAAFPAAABIgAAAZ8ALACVdAIAAaXgAAAAZAABrYEAAAD6AAAyyAAALeYAAbJ5AAHLJAABmigAAb7lACwAxhUCAASNAAAAADIABJbMAAABLAAAHIQAAByEAASLzwAEp6kABIvPAAR81AAcAAPpCQAXXOsAF3/IABcrBAAXTeEAF3A8///srwAsAMWzAgAAAHgAAAAyAAAAggAAMvoAAXNKAACPwAAAAKUAAAClAAAAcwAAAKoALADF1wIAA7JFAAAAMgAAAAAAAAAAAAAe3AAAHLYAAAAAAAAAAAAAAAAAAwwyACwA0RoCAAAHEgAAABkAAAgRABdmbgAAjQQAAGHzAAALvQAADBIAAAXmAAAKSwAsANEpAgAACloAAABkAAALzwCB2/IAAig/AAGxNAAAEQMAABJcAAAIZgAADnkALAC/6QIAAXDAAAAAMgABdygAABA2AABsmAAAVrgAAXj0AAGTJQABTDoAAWJgACwAxPgCAAJRwAAAAooAAlD3AAAD6AAAMZwAACn+AAJCEQACUcAAAkIRAAIFXQAsAMUyAgAAAKAAAAAyAAAArwAGj7AAAVeOAADTVAAAAOEAAADhAAAAlgAAANIALAC3lwIAAARgAAAAGQAABOIANNfuAAIDoAAAzH4AAAKoAAAJxAAAAqgAAABuACwAvZUCAAADogAAADIAAAOvAGHimAAFfRQAASJuAAABYwAABl4AAAFjAAAANwAsAL/cAgAAAu4AAAAyAAADowB0RHQAAmZCAAIH7AAABQUAAAZFAAACrQAABh0ALADRHgIAAAAAAAAAKAAAAAAAAAAAAAAZfQAAFo8AAAAAAAAAAAAAAAAABfc6ACwAxOsCAAAAuQAAASwAAADLAAjjZAABYFgAAODOAAAA5gAAAOsAAAC0AAAA+gAsANGnAgAB+LAAAAAyAAIKBAAAHpEAAI9DAAA2sAACOi0AAnVCAAG/lAACDg4ALAFCOgIACMAtAAAAMgAI1p4AAABkAAAOEAAADhAACO0PAAjtDwAIwC0ABT8WACwBQkQCAAAAAAAAAZcAAAAAAAAAAAAAFLQAABS0AAAAAAAAAAAAAAAAAAXn2wAsAL2CAgAAAAAAAAAoAAAAAAAAAAAAACowAAAjjAAAAAAAAAAAAAAAAAACLEoALADF/AIAAABkAAAAMgAAAGcAAGUsAAIPvAAAsVgAAACbAAAAmwAAAF8AAACbACwAsRwCAACLUQAAAPoAAJG4ADCqmAABX8IAARE+AACL+wAArvYAAHEgAACH8AAsAL/xAgABqoEAAAAyAAGtEgAAOgIAAHM8AABO6AABoVgAAc4rAAGGoAABnPgALADRhwIAAQLKAAAAGQABF8cACVCrAAC/mgAAgtwAAUFAAAF6OQAA5twAASNjACwA0c4CAAAEVgAAABkAAATrAB0QIwABNWAAAKyPAAADNAAAB3YAAAM0AAAF1wAsANSZAgAAB/MAAAAyAAAIawCbP8AAAXFWAAKY2AAACZIAAAtFAAAFjAAACAcALADGKwIAAAAAAAABkAAAAAAAAAAAAAAjWgAAHIQAAAAAAAAAAAAAAAAAAjuGACwAt5oCAAAAAAAAABkAAAAAAAAAAAAAE+wAABH4AAAAAAAAAAAAAAAAAAVCbQAsANGoAgAAFrwAAACWAAAXBAEUVNIAAyl5AAPwygAAEGgAACIGAAAL+QAAFo8ALACWwQIAAEYAAAAAMgAARoMBNHQQAAQAnAAD67YAAE9MAABbcgAANMsAAFDmACwA0fMCAAACAwAAABkAAAJEAAgG+wAA570AAEQqAAACYgAAAu4AAAHgAAADDAAsAIkCAgAAARgAAAAZAAABLgAAvqAAAHFhAAAXDAAAAakAAAGpAAABGAAAAXIALADUUQIAAY3pAAAAZAABioUAAAD6AAA6mAAAL0QAAYFpAAGN6QABgWkAAZ0qACwApaECAAAumgAAADIAADKbAd9sOgADWdAABmSGAAAqlAAAQAYAACMyAAAu7wAsAL/8AgAB/n0AAAAyAAH3+AAAD24AAEz0AABHfAACCzkAAhamAAHaVgAB5u8ALADRMgIAA2B0AAAAGQADap4AAACvAAAgOgAAJ78AA471AAOO9QADSL4ABPCNACwA0U0CAAA3vgAAABkAAEE2ADSWHAAA2vIAAOaRAABOcAAAY6YAAC4xAABGNwAsANI2AgAAAAAAAADIAAAAAAAAAAAAABS0AAAUtAAAAAAAAAAAAAAAAAAF50U=";
        let buf = bytes::Bytes::from(base64::decode(line)?);
        // println!("{}", buf.len());
        // println!("{:?}", &buf);
        if buf.len() > 2 {
            let tickers = Tickers::try_from(buf.as_ref())?;
            // print_tickers_struct(&tickers);
            pretty_print_tickers(&tickers);
            // write_json_file(&json_file, &tickers)?;
            // write_bin_file(&mut bin_file, &tickers)?;
        }
        Ok(())
    }
    #[test]
    fn test_tickers_three_thousand_quote_message() -> Result<(), Box<dyn std::error::Error>> {
        // let json_file = get_file("kite_tickers_three_thousand_quote_message", "json", (true, true, true))?;
        // let mut bin_file = get_file("kite_tickers_three_thousand_quote_message", "bin", (true, true, true))?;
        if let Ok(lines) = read_lines("./custom_mock_files/ws_three_thousand_quote_message.packet")
        {
            for line in lines {
                if let Ok(line) = line {
                    let buf = bytes::Bytes::from(base64::decode(line)?);
                    // println!("{}", buf.len());
                    // println!("{:?}", &buf);
                    if buf.len() > 2 {
                        let tickers = Tickers::try_from(buf.as_ref())?;
                        // print_tickers_struct(&tickers);
                        print_tickers(&tickers);
                        // pretty_print_tickers(&tickers);
                        // write_json_file(&json_file, &tickers)?;
                        // write_bin_file(&mut bin_file, &tickers)?;
                    }
                }
            }
        }
        Ok(())
    }
    #[test]
    fn test_tickers_extended_depth() -> Result<(), Box<dyn std::error::Error>> {
        let json_file = get_file("kite_tickers_extended_depth", "json", (true, true, true))?;
        let mut bin_file = get_file("kite_tickers_extended_depth", "bin", (true, true, true))?;
        if let Ok(lines) = read_lines("./custom_mock_files/ws_extended_depth.packet") {
            for line in lines {
                if let Ok(line) = line {
                    let buf = bytes::Bytes::from(base64::decode(line)?);
                    // println!("{}", buf.len());
                    // println!("{:?}", &buf);
                    if buf.len() > 2 {
                        let tickers = Tickers::try_from(buf.as_ref())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        write_json_file(&json_file, &tickers)?;
                        write_bin_file(&mut bin_file, &tickers)?;
                    }
                }
            }
        }
        Ok(())
    }
    #[test]
    fn test_tickers_extended_depth_one() -> Result<(), Box<dyn std::error::Error>> {
        let json_file = get_file(
            "kite_tickers_extended_depth_one",
            "json",
            (true, true, true),
        )?;
        let mut bin_file = get_file("kite_tickers_extended_depth_one", "bin", (true, true, true))?;
        if let Ok(lines) = read_lines("./custom_mock_files/ws_extended_depth_20.packet") {
            for line in lines {
                if let Ok(line) = line {
                    let buf = bytes::Bytes::from(base64::decode(line)?);
                    // println!("{}", buf.len());
                    // println!("{:?}", &buf);
                    if buf.len() > 2 {
                        let tickers = Tickers::try_from(buf.as_ref())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        write_json_file(&json_file, &tickers)?;
                        write_bin_file(&mut bin_file, &tickers)?;
                    }
                }
            }
        }
        Ok(())
    }
    #[test]
    fn test_tickers_three_thousand_extended_quote_message() -> Result<(), Box<dyn std::error::Error>>
    {
        let json_file = get_file(
            "kite_tickers_three_thousand_extended_quote_message",
            "json",
            (true, true, true),
        )?;
        let mut bin_file = get_file(
            "kite_tickers_three_thousand_extended_quote_message",
            "bin",
            (true, true, true),
        )?;
        if let Ok(lines) =
            read_lines("./custom_mock_files/ws_three_thousand_extended_quote_message.packet")
        {
            for line in lines {
                if let Ok(line) = line {
                    let buf = bytes::Bytes::from(base64::decode(line)?);
                    // println!("{}", buf.len());
                    // println!("{:?}", &buf);
                    if buf.len() > 2 {
                        let tickers = Tickers::try_from(buf.as_ref())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        write_json_file(&json_file, &tickers)?;
                        write_bin_file(&mut bin_file, &tickers)?;
                    }
                }
            }
        }
        Ok(())
    }
    #[test]
    fn test_tickers_nbn_idx_fo_full_quote_messages() -> Result<(), Box<dyn std::error::Error>> {
        let json_file = get_file(
            "test_tickers_nbn_idx_fo_full_quote_messages",
            "json",
            (true, true, true),
        )?;
        let mut bin_file = get_file(
            "test_tickers_nbn_idx_fo_full_quote_messages",
            "bin",
            (true, true, true),
        )?;
        if let Ok(lines) =
            read_lines("./custom_mock_files/ws_nifty_and_banknifty_22623_index_and_future_and_option_data_full_mode_22617.packet")
        {
            for line in lines {
                if let Ok(line) = line {
                    let buf = bytes::Bytes::from(base64::decode(line)?);
                    // println!("{}", buf.len());
                    // println!("{:?}", &buf);
                    if buf.len() > 2 {
                        let tickers = Tickers::try_from(buf.as_ref())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        write_json_file(&json_file, &tickers)?;
                        write_bin_file(&mut bin_file, &tickers)?;
                    }
                }
            }
        }
        Ok(())
    }
    #[test]
    fn test_tickers_nbn_idx_fo_extended_quote_messages() -> Result<(), Box<dyn std::error::Error>> {
        let json_file = get_file(
            "test_tickers_nbn_idx_fo_extended_quote_messages_1",
            "json",
            (true, true, true),
        )?;
        let mut bin_file = get_file(
            "test_tickers_nbn_idx_fo_extended_quote_messages_1",
            "bin",
            (true, true, true),
        )?;
        if let Ok(lines) =
            read_lines("./custom_mock_files/ws_nifty_and_banknifty_22623_future_and_option_data_extended_mode_22617.packet")
        {
            for line in lines {
                if let Ok(line) = line {
                    let buf = bytes::Bytes::from(base64::decode(line)?);
                    // println!("{}", buf.len());
                    // println!("{:?}", &buf);
                    if buf.len() > 2 {
                        let tickers = Tickers::try_from(buf.as_ref())?;
                        // print_tickers_struct(&tickers);
                        print_tickers(&tickers);
                        // pretty_print_tickers(&tickers);
                        // write_json_file(&json_file, &tickers)?;
                        // write_bin_file(&mut bin_file, &tickers)?;
                    }
                }
            }
        }
        Ok(())
    }
    #[test]
    fn test_tickers_nbn_idx_fo_extended_quote_messages_buffered(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json_file = get_file(
            "test_tickers_nbn_idx_fo_extended_quote_messages",
            "json",
            (true, true, true),
        )?;
        let mut bin_file = get_file(
            "test_tickers_nbn_idx_fo_extended_quote_messages",
            "bin",
            (true, true, true),
        )?;
        let mut file = read_file(
                "./custom_mock_files/ws_nifty_and_banknifty_22623_future_and_option_data_extended_mode_22617.packet",
            )?;
        let mut buf = String::new();
        while file.read_line(&mut buf)? > 0 {
            let data = bytes::Bytes::from(base64::decode(buf.trim())?);
            // println!("{}", data.len());
            // println!("{:?}", &data);
            if data.len() >= 2 {
                let tickers = Tickers::try_from(data.as_ref())?;
                // print_tickers_struct(&tickers);
                print_tickers(&tickers);
                // pretty_print_tickers(&tickers);
                // write_json_file(&json_file, &tickers)?;
                // write_bin_file(&mut bin_file, &tickers)?;
            }
            buf.clear();
        }
        Ok(())
    }
    #[test]
    fn test_tickers_nbn_idx_fo_extended_quote_messages_parallel(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json_file = get_file(
            "test_tickers_nbn_idx_fo_extended_quote_messages_par",
            "json",
            (true, true, true),
        )?;
        let mut bin_file = get_file(
            "test_tickers_nbn_idx_fo_extended_quote_messages_par",
            "bin",
            (true, true, true),
        )?;
        let num_threads = ::num_cpus::get();
        println!("num_threads: {}", num_threads);
        let mut file = read_file(
                "./custom_mock_files/ws_nifty_and_banknifty_22623_future_and_option_data_extended_mode_22617.packet",
            ).unwrap();
        let mut buf = String::new();
        let mut buf_array: Vec<String> = Vec::with_capacity(num_threads);
        let mut tickers_array: Arc<Mutex<Vec<Tickers>>> = Arc::new(Mutex::new(Vec::new()));
        // dbg!("buff-array_len : {}", buf_array.len());
        let mut nol = 0;
        loop {
            while buf_array.len() < num_threads {
                // println!("{}", &buf);
                match file.read_line(&mut buf) {
                    Ok(0) => {
                        println!("EOF");
                        break;
                    }
                    Ok(_) => {
                        buf_array.push(buf.clone());
                        buf.clear();
                        nol += 1;
                        // dbg!("buff-array_len : {}", buf_array.len());
                    }
                    Err(err) => {
                        return Err(format!("{err}").into());
                    }
                }
            }
            if buf_array.is_empty() {
                break;
            }
            // dbg!("{:#?}", &buf_array);
            buf_array
                .par_iter()
                .filter_map(|line| {
                    let binary_data = base64::decode(line.trim()).unwrap();
                    if binary_data.len() > 2 {
                        Some(bytes::Bytes::from(binary_data))
                    } else {
                        None
                    }
                })
                .filter_map(|data| Tickers::try_from(data.as_ref()).ok())
                .for_each(|tickers| {
                    // dbg!("{:#?}", &tickers);
                    tickers_array.lock().unwrap().push(tickers);
                    // print_tickers(&tickers);
                    // pretty_print_tickers(&tickers);
                    // write_json_file(&json_file, &tickers).ok();
                    // write_bin_file(&mut bin_file, &tickers).ok();
                });
            // buf_array = next_buf_array;
            buf_array.clear();
        }
        println!("nol: {}", nol);
        println!(
            "tickers_array_len : {}",
            tickers_array.lock().unwrap().len()
        );
        for tickers in tickers_array.lock().unwrap().iter() {
            write_json_file(&json_file, tickers)?;
            write_bin_file(&mut bin_file, tickers)?;
        }
        Ok(())
    }
}
