use super::common::*;
use super::tickers::*;
use crate::utils::*;
use bytes_cast::{unaligned, BytesCast};
use chrono::{DateTime, FixedOffset, TimeZone};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::sync::{Arc, Mutex};

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct LtpBytesCast {
    pub token: unaligned::I32Be,
    pub last_price: unaligned::I32Be,
}

impl From<LtpBytesCast> for Ltp {
    fn from(ltp: LtpBytesCast) -> Self {
        let token = ltp.token.get();
        let segment = get_segment(token);
        let last_price = convert_price(segment, ltp.last_price.get());
        Self {
            mode: TickerStreamingMode::Ltp,
            is_tradable: is_tradable(segment),
            token,
            last_price,
        }
    }
}

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct LtpcBytesCast {
    pub token: unaligned::I32Be,
    pub last_price: unaligned::I32Be,
    pub close_price: unaligned::I32Be,
}

impl From<LtpcBytesCast> for Ltpc {
    fn from(ltpc: LtpcBytesCast) -> Self {
        let token = ltpc.token.get();
        let segment = get_segment(token);
        let last_price = convert_price(segment, ltpc.last_price.get());
        let close_price = convert_price(segment, ltpc.close_price.get());
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

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct QuoteIndexBytesCast {
    pub token: unaligned::I32Be,
    pub last_price: unaligned::I32Be,
    pub high_price: unaligned::I32Be,
    pub low_price: unaligned::I32Be,
    pub open_price: unaligned::I32Be,
    pub close_price: unaligned::I32Be,
    pub change_from_tick_packet: unaligned::I32Be,
}

impl From<QuoteIndexBytesCast> for QuoteIndex {
    fn from(quote_index: QuoteIndexBytesCast) -> Self {
        let token = quote_index.token.get();
        let segment = get_segment(token);
        let last_price = convert_price(segment, quote_index.last_price.get());
        let high_price = convert_price(segment, quote_index.high_price.get());
        let low_price = convert_price(segment, quote_index.low_price.get());
        let open_price = convert_price(segment, quote_index.open_price.get());
        let close_price = convert_price(segment, quote_index.close_price.get());
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
            change_from_tick_packet: quote_index.change_from_tick_packet.get() as f64,
        }
    }
}

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct FullIndexBytesCast {
    pub token: unaligned::I32Be,
    pub last_price: unaligned::I32Be,
    pub high_price: unaligned::I32Be,
    pub low_price: unaligned::I32Be,
    pub open_price: unaligned::I32Be,
    pub close_price: unaligned::I32Be,
    pub change_from_tick_packet: unaligned::I32Be,
    pub exchange_timestamp: unaligned::I32Be,
}

impl From<FullIndexBytesCast> for FullIndex {
    fn from(full_index: FullIndexBytesCast) -> Self {
        let token = full_index.token.get();
        let segment = get_segment(token);
        let last_price = convert_price(segment, full_index.last_price.get());
        let high_price = convert_price(segment, full_index.high_price.get());
        let low_price = convert_price(segment, full_index.low_price.get());
        let open_price = convert_price(segment, full_index.open_price.get());
        let close_price = convert_price(segment, full_index.close_price.get());
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
            exchange_timestamp: FixedOffset::east(19800)
                .timestamp(full_index.exchange_timestamp.get() as i64, 0),
            change_from_tick_packet: full_index.change_from_tick_packet.get() as f64,
        }
    }
}

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct QuoteBytesCast {
    pub token: unaligned::I32Be,
    pub last_price: unaligned::I32Be,
    pub last_quantity: unaligned::I32Be,
    pub average_price: unaligned::I32Be,
    pub volume: unaligned::I32Be,
    pub total_buy_quantity: unaligned::I32Be,
    pub total_sell_quantity: unaligned::I32Be,
    pub open_price: unaligned::I32Be,
    pub high_price: unaligned::I32Be,
    pub low_price: unaligned::I32Be,
    pub close_price: unaligned::I32Be,
}

impl From<QuoteBytesCast> for Quote {
    fn from(quote: QuoteBytesCast) -> Self {
        let token = quote.token.get();
        let segment = get_segment(token);
        let last_price = convert_price(segment, quote.last_price.get());
        let close_price = convert_price(segment, quote.close_price.get());
        let open_price = convert_price(segment, quote.open_price.get());
        let high_price = convert_price(segment, quote.high_price.get());
        let low_price = convert_price(segment, quote.low_price.get());
        let average_price = convert_price(segment, quote.average_price.get());
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
            volume: quote.volume.get(),
            last_quantity: quote.last_quantity.get(),
            total_buy_quantity: quote.total_buy_quantity.get(),
            total_sell_quantity: quote.total_sell_quantity.get(),
            average_price,
            open_price,
            high_price,
            low_price,
        }
    }
}

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct FullBytesCast {
    pub token: unaligned::I32Be,
    pub last_price: unaligned::I32Be,
    pub last_quantity: unaligned::I32Be,
    pub average_price: unaligned::I32Be,
    pub volume: unaligned::I32Be,
    pub total_buy_quantity: unaligned::I32Be,
    pub total_sell_quantity: unaligned::I32Be,
    pub open_price: unaligned::I32Be,
    pub high_price: unaligned::I32Be,
    pub low_price: unaligned::I32Be,
    pub close_price: unaligned::I32Be,
    pub last_traded_time: unaligned::I32Be,
    pub oi: unaligned::I32Be,
    pub oi_day_high: unaligned::I32Be,
    pub oi_day_low: unaligned::I32Be,
    pub exchange_timestamp: unaligned::I32Be,
    pub depth: MarketDepthBytesCast,
}

impl From<FullBytesCast> for Full {
    fn from(full: FullBytesCast) -> Self {
        let token = full.token.get();
        let segment = get_segment(token);
        let last_price = convert_price(segment, full.last_price.get());
        let close_price = convert_price(segment, full.close_price.get());
        let open_price = convert_price(segment, full.open_price.get());
        let high_price = convert_price(segment, full.high_price.get());
        let low_price = convert_price(segment, full.low_price.get());
        let average_price = convert_price(segment, full.average_price.get());
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        let depth = MarketDepth {
            buy: [0_u8; 5]
                .into_iter()
                .enumerate()
                .map(|(i, _)| {
                    let market_depth_data = MarketDepthData {
                        quantity: full.depth.buy[i].quantity.get(),
                        price: convert_price(segment, full.depth.buy[i].price.get()),
                        orders: full.depth.buy[i].orders.get() as i32,
                    };
                    market_depth_data
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
            sell: [0_u8; 5]
                .into_iter()
                .enumerate()
                .map(|(i, _)| {
                    let market_depth_data = MarketDepthData {
                        quantity: full.depth.sell[i].quantity.get(),
                        price: convert_price(segment, full.depth.sell[i].price.get()),
                        orders: full.depth.sell[i].orders.get() as i32,
                    };
                    market_depth_data
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
            change: price_change.change,
            absolute_change: price_change.absolute_change,
            open_change: price_change.open_change,
            open_change_percent: price_change.open_change_percent,
            tick_change: price_change.tick_change,
            volume: full.volume.get(),
            last_quantity: full.last_quantity.get(),
            total_buy_quantity: full.total_buy_quantity.get(),
            total_sell_quantity: full.total_sell_quantity.get(),
            average_price,
            open_price,
            high_price,
            low_price,
            depth,
            last_traded_time: FixedOffset::east(19800)
                .timestamp(full.last_traded_time.get() as i64, 0),
            oi: full.oi.get(),
            oi_day_low: full.oi_day_low.get(),
            oi_day_high: full.oi_day_high.get(),
            exchange_timestamp: FixedOffset::east(19800)
                .timestamp(full.exchange_timestamp.get() as i64, 0),
        }
    }
}

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct CompactFullBytesCast {
    pub token: unaligned::I32Be,
    pub last_price: unaligned::I32Be,
    pub last_quantity: unaligned::I32Be,
    pub average_price: unaligned::I32Be,
    pub volume: unaligned::I32Be,
    pub total_buy_quantity: unaligned::I32Be,
    pub total_sell_quantity: unaligned::I32Be,
    pub open_price: unaligned::I32Be,
    pub high_price: unaligned::I32Be,
    pub low_price: unaligned::I32Be,
    pub close_price: unaligned::I32Be,
    pub depth: MarketDepthBytesCast,
}

impl From<CompactFullBytesCast> for CompactFull {
    fn from(compact_full: CompactFullBytesCast) -> Self {
        let token = compact_full.token.get();
        let segment = get_segment(token);
        let last_price = convert_price(segment, compact_full.last_price.get());
        let close_price = convert_price(segment, compact_full.close_price.get());
        let open_price = convert_price(segment, compact_full.open_price.get());
        let high_price = convert_price(segment, compact_full.high_price.get());
        let low_price = convert_price(segment, compact_full.low_price.get());
        let average_price = convert_price(segment, compact_full.average_price.get());
        let price_change: PriceChange =
            PriceChange::calculate_price_change(last_price, open_price, close_price);
        let depth = MarketDepth {
            buy: [0_u8; 5]
                .into_iter()
                .enumerate()
                .map(|(i, _)| {
                    let market_depth_data = MarketDepthData {
                        quantity: compact_full.depth.buy[i].quantity.get(),
                        price: convert_price(segment, compact_full.depth.buy[i].price.get()),
                        orders: compact_full.depth.buy[i].orders.get() as i32,
                    };
                    market_depth_data
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
            sell: [0_u8; 5]
                .into_iter()
                .enumerate()
                .map(|(i, _)| {
                    let market_depth_data = MarketDepthData {
                        quantity: compact_full.depth.sell[i].quantity.get(),
                        price: convert_price(segment, compact_full.depth.sell[i].price.get()),
                        orders: compact_full.depth.sell[i].orders.get() as i32,
                    };
                    market_depth_data
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
            change: price_change.change,
            absolute_change: price_change.absolute_change,
            open_change: price_change.open_change,
            open_change_percent: price_change.open_change_percent,
            tick_change: price_change.tick_change,
            volume: compact_full.volume.get(),
            last_quantity: compact_full.last_quantity.get(),
            total_buy_quantity: compact_full.total_buy_quantity.get(),
            total_sell_quantity: compact_full.total_sell_quantity.get(),
            average_price,
            open_price,
            high_price,
            low_price,
            depth,
        }
    }
}

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ExtendedDepthBytesCast {
    pub token: unaligned::I32Be,
    pub last_price: unaligned::I32Be,
    pub last_traded_time: unaligned::I32Be,
    pub extended_depth: ExtendedMarketDepthBytesCast,
}

impl From<ExtendedDepthBytesCast> for ExtendedDepth {
    fn from(extended_depths: ExtendedDepthBytesCast) -> Self {
        let token = extended_depths.token.get();
        let segment = get_segment(token);
        let last_price = convert_price(segment, extended_depths.last_price.get());
        let last_traded_time = extended_depths.last_traded_time.get() as i64;
        let extended_depth = ExtendedMarketDepth {
            buy: [0_u8; 20]
                .into_iter()
                .enumerate()
                .map(|(i, _)| {
                    let market_depth_data = MarketDepthData {
                        quantity: extended_depths.extended_depth.buy[i].quantity.get(),
                        price: convert_price(
                            segment,
                            extended_depths.extended_depth.buy[i].price.get(),
                        ),
                        orders: extended_depths.extended_depth.buy[i].orders.get(),
                    };
                    market_depth_data
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
            sell: [0_u8; 20]
                .into_iter()
                .enumerate()
                .map(|(i, _)| {
                    let market_depth_data = MarketDepthData {
                        quantity: extended_depths.extended_depth.sell[i].quantity.get(),
                        price: convert_price(
                            segment,
                            extended_depths.extended_depth.sell[i].price.get(),
                        ),
                        orders: extended_depths.extended_depth.sell[i].orders.get(),
                    };
                    market_depth_data
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

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct MarketDepthBytesCast {
    pub buy: [MarketDepthDataBytesCast; 5],
    pub sell: [MarketDepthDataBytesCast; 5],
}

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ExtendedMarketDepthBytesCast {
    pub buy: [ExtendedMarketDepthDataBytesCast; 20],
    pub sell: [ExtendedMarketDepthDataBytesCast; 20],
}

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct MarketDepthDataBytesCast {
    pub quantity: unaligned::I32Be,
    pub price: unaligned::I32Be,
    pub orders: unaligned::I16Be,
    pub padding: unaligned::I16Be,
}

#[derive(BytesCast, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ExtendedMarketDepthDataBytesCast {
    pub quantity: unaligned::I32Be,
    pub price: unaligned::I32Be,
    pub orders: unaligned::I32Be,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TickerTypeDataBytesCast {
    LtpBytesCast(LtpBytesCast),
    LtpcBytesCast(LtpcBytesCast),
    QuoteIndexBytesCast(QuoteIndexBytesCast),
    FullIndexBytesCast(FullIndexBytesCast),
    QuoteBytesCast(QuoteBytesCast),
    FullBytesCast(FullBytesCast),
    CompactFullBytesCast(CompactFullBytesCast),
    ExtendedDepthBytesCast(ExtendedDepthBytesCast),
}

impl TickerTypeDataBytesCast {
    fn try_from_bytes_cast(
        (packet_length, input): (usize, &[u8]),
    ) -> Result<(Self, &[u8]), Box<dyn std::error::Error>> {
        match packet_length {
            8 => {
                let (ltp, rest) = LtpBytesCast::from_bytes(input).unwrap();

                Ok((Self::LtpBytesCast(*ltp), rest))
            }
            12 => {
                let (ltpc, rest) = LtpcBytesCast::from_bytes(input).unwrap();

                Ok((Self::LtpcBytesCast(*ltpc), rest))
            }
            28 => {
                let (quote_index, rest) = QuoteIndexBytesCast::from_bytes(input).unwrap();

                Ok((Self::QuoteIndexBytesCast(*quote_index), rest))
            }
            32 => {
                let (full_index, rest) = FullIndexBytesCast::from_bytes(input).unwrap();

                Ok((Self::FullIndexBytesCast(*full_index), rest))
            }
            44 => {
                let (quote, rest) = QuoteBytesCast::from_bytes(input).unwrap();

                Ok((Self::QuoteBytesCast(*quote), rest))
            }
            164 => {
                let (compact_full, rest) = CompactFullBytesCast::from_bytes(input).unwrap();

                Ok((Self::CompactFullBytesCast(*compact_full), rest))
            }
            184 => {
                let (full, rest) = FullBytesCast::from_bytes(input).unwrap();

                Ok((Self::FullBytesCast(*full), rest))
            }
            492 => {
                let (extended_depth, rest) = ExtendedDepthBytesCast::from_bytes(input).unwrap();

                Ok((Self::ExtendedDepthBytesCast(*extended_depth), rest))
            }
            _ => Err("Invalid Input Length".into()),
        }
    }
}

impl TryFrom<&[u8]> for TickerTypeDataBytesCast {
    type Error = Box<dyn std::error::Error>;
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        match input.len() {
            8 => {
                let (ltp, rest) = LtpBytesCast::from_bytes(input).unwrap();

                Ok(Self::LtpBytesCast(*ltp))
            }
            12 => {
                let (ltpc, rest) = LtpcBytesCast::from_bytes(input).unwrap();

                Ok(Self::LtpcBytesCast(*ltpc))
            }
            28 => {
                let (quote_index, rest) = QuoteIndexBytesCast::from_bytes(input).unwrap();

                Ok(Self::QuoteIndexBytesCast(*quote_index))
            }
            32 => {
                let (full_index, rest) = FullIndexBytesCast::from_bytes(input).unwrap();

                Ok(Self::FullIndexBytesCast(*full_index))
            }
            44 => {
                let (quote, rest) = QuoteBytesCast::from_bytes(input).unwrap();

                Ok(Self::QuoteBytesCast(*quote))
            }
            164 => {
                let (compact_full, rest) = CompactFullBytesCast::from_bytes(input).unwrap();

                Ok(Self::CompactFullBytesCast(*compact_full))
            }
            184 => {
                let (full, rest) = FullBytesCast::from_bytes(input).unwrap();

                Ok(Self::FullBytesCast(*full))
            }
            492 => {
                let (extended_depth, rest) = ExtendedDepthBytesCast::from_bytes(input).unwrap();

                Ok(Self::ExtendedDepthBytesCast(*extended_depth))
            }
            _ => Err("Invalid Input Length".into()),
        }
    }
}

impl TryFrom<(usize, TickerTypeDataBytesCast)> for TickersData {
    type Error = &'static str;

    fn try_from(
        (packet_length, ticker_type_data): (usize, TickerTypeDataBytesCast),
    ) -> Result<Self, Self::Error> {
        let ticker_type = TickerType::try_from(packet_length as usize)?;
        let ticker_type_data = TickerTypeData::try_from((ticker_type, ticker_type_data))?;
        Ok(Self {
            ticker_type,
            ticker_type_data,
        })
    }
}

impl TryFrom<(TickerType, TickerTypeDataBytesCast)> for TickerTypeData {
    type Error = &'static str;
    fn try_from(
        (ticker_type, ticker_type_data): (TickerType, TickerTypeDataBytesCast),
    ) -> Result<Self, Self::Error> {
        match (ticker_type, ticker_type_data) {
            (TickerType::Ltp, TickerTypeDataBytesCast::LtpBytesCast(ltp_bytes_cast)) => Ok(Self::from(Ltp::from(ltp_bytes_cast))),
            (TickerType::Ltpc, TickerTypeDataBytesCast::LtpcBytesCast(ltpc_bytes_cast)) => Ok(Self::from(Ltpc::from(ltpc_bytes_cast))),
            (TickerType::QuoteIndex, TickerTypeDataBytesCast::QuoteIndexBytesCast(quote_index_bytes_cast)) => Ok(Self::from(QuoteIndex::from(quote_index_bytes_cast))),
            (TickerType::FullIndex, TickerTypeDataBytesCast::FullIndexBytesCast(full_index_bytes_cast)) => Ok(Self::from(FullIndex::from(full_index_bytes_cast))),
            (TickerType::Quote, TickerTypeDataBytesCast::QuoteBytesCast(quote_bytes_cast)) => Ok(Self::from(Quote::from(quote_bytes_cast))),
            (TickerType::Full, TickerTypeDataBytesCast::FullBytesCast(full_bytes_cast)) => Ok(Self::from(Full::from(full_bytes_cast))),
            (TickerType::CompactFull, TickerTypeDataBytesCast::CompactFullBytesCast(compact_full_bytes_cast)) => Ok(Self::from(CompactFull::from(compact_full_bytes_cast))),
            (TickerType::ExtendedDepth, TickerTypeDataBytesCast::ExtendedDepthBytesCast(extended_depth_bytes_cast)) => Ok(Self::from(ExtendedDepth::from(extended_depth_bytes_cast))),
            (_, _) => Err("TickerTypeData Only Accepts TickerType::Ltp, Ltpc, QuoteIndex, FullIndex, Quote, Full, CompactFull, ExtendedDepth, FullExtendedDepth !"),
        }
    }
}

impl TryFrom<&str> for Tickers {
    type Error = Box<dyn std::error::Error>;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let input = &base64::decode(line)?[..];
        // println!("{}", input.len());
        // println!("{:?}", &input);
        match input {
            input if input.len() > 2 => {
                let (tickers, mut rest) = unaligned::I16Be::from_bytes(input).unwrap();
                // assert_eq!(rest.len(), tickers as usize);
                let tickers_data = vec![0_u8; tickers.get() as usize]
                    .iter()
                    .map(|_| {
                        let (packet_length, _) =
                            unaligned::I16Be::from_bytes(rest.take(..2).unwrap()).unwrap();
                        // dbg!(packet_length.get() as usize, rest.len());
                        let packet_data = rest.take(..packet_length.get() as usize).unwrap();
                        // dbg!(packet_data.len(), rest.len());
                        (packet_length.get() as usize, packet_data)
                    })
                    .par_bridge()
                    .map(|(packet_length, packet_data)| {
                        let ticker_type_data =
                            TickerTypeDataBytesCast::try_from(packet_data).unwrap();
                        let tikers_data =
                            TickersData::try_from((packet_length, ticker_type_data)).unwrap();
                        tikers_data
                    })
                    .collect::<Vec<TickersData>>();
                Ok(Self {
                    tickers: tickers.get(),
                    tickers_data,
                })
            }
            _ => Err("Invalid Input Length".into()),
        }
    }
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
                    if line.as_str() != "AA==" {
                        let tickers = Tickers::try_from(line.as_str())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        print_tickers(&tickers);
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
                "AbIALACJbgIAAAE7AAAAGQAAAWAAACLEAABf/wAAFDcAAAGQAAABqQAAATsAAAGkACwAtAsCAAAZRgAAAJYAABtzAGUrBgACSfAAAj44AAAa9AAAI2QAABJ6AAAZvgAsANFYAgAAVnIAAAAZAABkUwGU4O0AAvXQAAV2DAAAdUQAAJRwAABI+AAAaVoALADR1gIAAANrAAAAGQAAA+IAGRrCAAFFGQAAawgAAASwAAAFtAAAAtAAAATxACwA0jQCAAZ99AAAABkABsv2AAABRQAAHycAACAhAAby6AAHGGgABmnMAAa4tAAsAImVAgAAAAAAAACWAAAAAAAAAAAAAA4QAAAOEAAAAAAAAAAAAAAAAAAGOBcALADFqAIAA1CxAAAAZAAAAAAAAAAAAAAjWgAAI1oAAAAAAAAAAAAAAAAAAwN8ACwAxhgCAAAAWgAAAGQAAABiAAJSiAACh9oAAKLkAAAAbgAAAH0AAABaAAAAoAAsANENAgAAAAAAAAAyAAAAAAAAAAAAABeiAAAYnAAAAAAAAAAAAAAAAAAGXzYALADSCwIABP7jAAAAMgAFGGEAAAO2AAAxOAAAMQYABYbYAAWG2AAE5DoABR1HACwA1FUCAAAAAAAAAfQAAAAAAAAAAAAALq4AADWEAAAAAAAAAAAAAAAAAAJr5wAsALeWAgAAAAAAAAAyAAAAAAAAAAAAABw5AAAaRQAAAAAAAAAAAAAAAAAFkC4ALADRLQIAAAyKAAAAGQAADiIAF/BrAADTbQAAhX8AABMpAAAWbAAACfYAABGoACwApZMCAAC/5QAAADIAALoEAAOp5AABOXoAAMmQAADDHgAA33oAAJ9CAADLGwAsAMYsAgAAAFUAAAV4AAAAWgADlTAAA4CuAAF3yAAAAIwAAACMAAAASwAAAIIALADSHAIABbkoAAAAGQAAAAAAAAAAAAASKgAAFRgAAAAAAAAAAAAAAAAAAtvgACwAxZQCAAAAhwAAADIAAACPAAV2ogAD0MIAAKeUAAAAtAAAALkAAAB9AAAApQAsAMYgAgAAAF8AAAPoAAAAagAAUwIAA4l4AABaCgAAALQAAAC0AAAAWgAAAL4ALADRLgIAAAAAAAACCAAAAAAAAAAAAAAaEwAAGfoAAAAAAAAAAAAAAAAABS7CACwA0TcCAANACAAAABkAAuVrAAAmxQAAfkUAACw9AAKwVwADUycAApc5AASvgwAsANGCAgAAbXQAAACvAABpggCAL6AAARqeAAGN8wAAX6oAAImyAABEjgAAZgMALADRogIAABueAAAAZAAAG4EATHcZAADIGQAA2IEAAA6wAAAoVQAADrAAABrHACwAsQ8CAABmbAAAAMgAAGwvACE9kgACl94AAYPkAABmsgAAhX8AAFHHAABlIgAsAL/9AgAAAMgAAAAyAAAA1wAQCOIAAVm0AAEslgAAASIAAAEiAAAAlgAAAQ4ALADRdwIAAKGfAAAAGQAAsRsA+O1LAALBeAAC40IAANrFAAD96AAAjKUAALxNACwAvZsCAADs4AAAADIAAO6EAAAOQgAAtwIAAFB4AADmcwABDRAAANLwAAD7GAAsAL/zAgAByXsAAAAyAAHC5gAAAV4AAFJsAABOhAABz0MAAc9DAAGzGQABrJMALACPwAIAAAAAAAAPoAAAAAAAAAAAAAATCwAAEwsAAAAAAAAAAAAAAAAABhruACwApZgCAACg1wAAADIAAJujAC4YMgAEcLgAATbwAACivAAAvm4AAIL6AACrqQAsANFFAgACXkAAAABkAAIEMgAABdwAAGlGAAA0DQACEfEAAmGwAAHYFwADdq4ALADRsQIAAm9/AAAAGQAComsAAA0WAACGLgAAO8QAAt5gAALkGQACIpoAAnwEACwAj7MCAAAAAAAAABkAAAAAAAAAAAAAEB0AABAdAAAAAAAAAAAAAAAAAAX30AAsAL/iAgAAAeUAAAAyAAACUgBr6AYABMg4AAHDLAAAA2sAAAO2AAAB0QAAA+gALADGJwIABRrCAAAAZAAFIwAAAADIAAAchAAAHIQABSs+AAUrPgAFGsIAAiyLACwA0fACAARKSAAAADIABGcAAAAAfQAAGqkAAByEAASXRgAEl0YABD/kAASPEgAsANISAgAFWGEAAAAZAAVYYQAAABkAAA4QAAAOEAAFWGEABVhhAAVYYQAEF0QALACJYwIAAAAAAAAAMgAAAAAAAAAAAAAOEAAADhAAAAAAAAAAAAAAAAAABbBZACwAlXUCAAAEqwAAADIAAATQAOrtNgAGe/YAA0AIAAAD6AAABl4AAAMvAAAE2AAsAL/XAgAA6NoAAAAyAADwagADem4AAMR8AABt9gAA5N4AARQxAADJLAAA4qkALACIzwIAB7OGAAAAGQAAAAAAAAAAAAAUtAAAFLQAAAAAAAAAAAAAAAAAB5SvACwAiWsCAAAAAAAAABkAAAAAAAAAAAAAFLQAABS0AAAAAAAAAAAAAAAAAAXzwAAsAKWWAgAArm8AAAAyAACq3QAAp5QAAQMuAAC4YAAAttUAAMuOAACS/gAAvbUALACY5gIAAMilAAAAMgAA0A8AEi/cAADLIAAAwiQAAMCUAADweAAAp8YAAMFrACwA0RQCAAAGlQAAAEsAAAd/AEUr6QAA7pMAAK27AAALHQAACx0AAAV9AAAJZQAsANIdAgAAAYsAAAAZAAABrAAQhw4AAX9/AABeCwAAAV4AAAINAAABXgAAAiEALACvYQIAAaJwAAAAMgABnyMAAACWAAA4pAAANIoAAZNmAAGnkwABk2YAAltNACwApaACAACDnwAABH4AAH+aAC0w8gADU5AAART0AACH6wAAn0cAAGjxAACOXQAsAJa+AgAAQfAAAAAyAABIYgHQXF4AAsLWAAVsegAARloAAFkGAAAy0gAAQfAALADFMQIAAo5RAAAAZAAClPAAACBsAABMkAAARewAAoo8AAK4nwACcWQAAoDSACwAxioCAAAAWgAAAGQAAABZAAFHvAABreIAAJ7KAAAAlgAAAJYAAABLAAAAjAAsANErAgAAC2gAAAAZAAAM2wAXcH0AAOouAABklgAAEG0AABQjAAAJJAAAD6UALAC/7gIAAAETAAAAMgAAAUcAMCUyAAKMJgABbswAAAGpAAABxwAAARMAAAHlACwAxTUCAAKhGwAAADIAAAAAAAAAAAAAJ0IAAB+kAAAAAAAAAAAAAAAAAAIDoAAsANE6AgAAF4kAAAAZAAAbkAAhH+IAALE/AABvhgAAIygAACpdAAASwAAAH8wALADRgQIAAOwdAAAAGQAA/ykAWOKwAAHzHwABJwUAATgXAAFd4gAA0aEAAQh0ACwA0i0CAAABcgAAABkAAAGJAAILVwAAwuwAAB1MAAABqQAAAg0AAAFZAAAB6gAsANSgAgABJiQAAABkAAEc+AAAfDgAAKCMAABUxAABFdAAAUcmAAECrAABPWwAHAAD+AkAMerAADIZQQAxRX8AMW2xADHFDgAAJbIALADRQQIAAnM6AAAAMgACOw8AAAF3AABoTAAAMksAAi1YAAJzOgACHV4AAw2kACwAxhkCAATQCAAAAGQABMMuAAAAyAAAHIQAAByEAAS2VAAE0AgABLZUAAHWDwAsANE8AgAAGfAAAAAZAAAesAAgDwgAAJJKAABmPwAAKjAAAC92AAAU8AAAI9IALADR+QIAAAH0AAAAGQAAAiIABZieAADSQQAATugAAAIhAAACvAAAAcwAAALVACwA0jcCAAABXgAAADIAAAGFAADWvwAAX80AABJDAAABwgAAAcwAAAFeAAAB1gAsAL1/AgAAAeUAAADIAAAB6ABTRfgACodQAAJOPAAAATYAAAKAAAABNgAAAAoALACWuwIAAGjsAAABXgAAZRcBLlc4AATHPgAC0B4AAGXMAACCFAAAUZ8AAHORACwA0c0CAAM0UAAAABkAA2IMAAABdwAAPk4AACpiAAPCHAADwiEAAyGVAANJzAAsALEQAgAAKo8AAABkAAAq8gORwhIACElKAAttyAAALKYAADu6AAAfkAAANFgALADGFAIAAABkAAAAMgAAAGAAARQsAAOeXgAAnjQAAACHAAAAhwAAAF8AAACgACwA0TgCAAAVGAAAABkAABhbABdkYQAAnekAAGDgAAAc3gAAJhYAABDMAAAb7gAsANFMAgAB5FsAAAAZAAG49AAASoMAANfSAABBCgABswAAAhgnAAF3yAABxoMALACIxAIAAAFKAAAAGQAAAW8AADLIAABULgAAEioAAAGaAAABqQAAAUoAAAHRACwA1KECAAANVwAAADIAAA5fAMwsjgABuDwAAkJSAAAOpgAAE3kAAAmDAAANcAAsAMUwAgAAAJsAAABkAAAAsgCe6HgATX7eAAfaWgAAALQAAADrAAAAhwAAANIALACPtAIAAAEEAAAAGQAAAR4AECC2AANCkgACbl0AAAExAAABfAAAAP8AAAF8ACwA0WACAAEaIQAAABkAAPvWAD9y3wABxpcAAISeAADfLwABQqgAAMYMAAEGgAAsAL/jAgABS0AAAAAyAAFMGgAABUYAAGe2AABKBgABXHUAAVx1AAE3nwABPgwALACI9gIAAAETAAAAGQAAASoAABGtAADNRgAADvEAAAExAAABNgAAAPAAAAGpACwA1JsCAAAJHwAAADIAAAmyAC423AAA/bYAAUoUAAALSgAADOQAAAZeAAAJJAAsANIuAgAGdFgAAABLAAZ0WAAAAdsAABLA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VeUALADdnQIAMfv/AAAAGQAx2T4ALlYcAAHuugACe1oAMZdBADIylgAxVtwAMcuPACwA0UICAAAgewAAABkAACYGACmzfQAAujsAAJ5/AAAw6AAAO7UAABoxAAApuAAsANFcAgABMu8AAAAyAAEWVQAYmK4AAdYeAACDDgAA/coAAV6vAADcvgABHjEALADRvwIAAs8BAAAAGQAC9CgAADJkAACDJwAAOuMAAxNdAANS1wACjlEAAuluACwAvXgCAAIlYAAAADIAAkU/AAABkAAANuIAACjSAAJMnQACW44AAiVgAAJTRgAsALEZAgAAIygAAAAyAAAjbQDRD+wAAqReAAMEEgAAKNwAADK5AAAaGAAALHkALADSLAIABhq8AAAAGQAGGrwAAABLAAAUtAAAFLQABhq8AAYavAAGGrwABklgACwBQjsCAAABLAAAABkAAAFPAAHUKgABDSQAACU1AAABXgAAAeAAAAEnAAABmgAsAMXNAgAAAHgAAAAyAAAAfQABbaAAAZImAACQugAAAKUAAAC+AAAAcwAAAKUALADF4gIAAABpAAAAyAAAAHAAAk48AAFcPgAArQwAAACbAAAAmwAAAGkAAACbACwAt5ECAAADwAAAAGQAAAQeAAtfhgAA+3cAADRxAAABlQAABjYAAAGVAAAAQQAsAMX0AgAEJ1cAAAAyAAAAAAAAAAAAACNaAAAchAAAAAAAAAAAAAAAAAADjDQALAC3jwIAAANwAAAAyAAAA9YADBTGAAECmAAAJmEAAAHWAAAExAAAAdYAAAAyACwA0VoCAABgHQAAABkAAHDzAIkIwwABVZoAApU7AACI2wAAo8oAAFIIAABzVQAsANF6AgAApBAAAAAZAACewAEcCE4AAgehAAM0zQAAkncAAMWyAABq0QAAmn4ALACWxAIAAFwhAAAAMgAAYGEBS6XCAAQtiAACnPIAAGGoAAB5GAAASNoAAFtAACwAxTQCAAAAlgAAADIAAACoAAbVBgADhJYAARP6AAAA+gAAAPoAAACRAAAAyAAsANH9AgAEzJ0AAAAZAATQCwAAAMgAABc+AAAUtAAE/iAABP4gAAS/LQAEKHQALAFCPwIAAAE2AAAAGQAAAUwAAE9lAABnhAAAERcAAAGfAAABqQAAATEAAAGfACwA1FYCAAAHAwAAADIAAAdXAChV+gAA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";
        let tickers = Tickers::try_from(line)?;
        // print_tickers_struct(&tickers);
        // pretty_print_tickers(&tickers);
        print_tickers(&tickers);
        // write_json_file(&json_file, &tickers)?;
        // write_bin_file(&mut bin_file, &tickers)?;
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
                    if line.as_str() != "AA==" {
                        let tickers = Tickers::try_from(line.as_str())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        print_tickers(&tickers);
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
                    if line.as_str() != "AA==" {
                        let tickers = Tickers::try_from(line.as_str())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        print_tickers(&tickers);
                        // write_json_file(&json_file, &tickers)?;
                        // write_bin_file(&mut bin_file, &tickers)?;
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
                    if line.as_str() != "AA==" {
                        let tickers = Tickers::try_from(line.as_str())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        print_tickers(&tickers);
                        // write_json_file(&json_file, &tickers)?;
                        // write_bin_file(&mut bin_file, &tickers)?;
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
                    if line.as_str() != "AA==" {
                        let tickers = Tickers::try_from(line.as_str())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        print_tickers(&tickers);
                        // write_json_file(&json_file, &tickers)?;
                        // write_bin_file(&mut bin_file, &tickers)?;
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
                    if line.as_str() != "AA==" {
                        let tickers = Tickers::try_from(line.as_str())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        print_tickers(&tickers);
                        // write_json_file(&json_file, &tickers)?;
                        // write_bin_file(&mut bin_file, &tickers)?;
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
                    if line.as_str() != "AA==" {
                        let tickers = Tickers::try_from(line.as_str())?;
                        // print_tickers_struct(&tickers);
                        // pretty_print_tickers(&tickers);
                        print_tickers(&tickers);
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
            if buf.trim() != "AA==" {
                let tickers = Tickers::try_from(buf.trim())?;
                // print_tickers_struct(&tickers);
                // pretty_print_tickers(&tickers);
                print_tickers(&tickers);
                // write_json_file(&json_file, &tickers)?;
                // write_bin_file(&mut bin_file, &tickers)?;
                buf.clear();
            }
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
                    "./custom_mock_files/ws_nifty_and_banknifty_22623_index_and_future_and_option_data_full_mode_22617.packet",
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
                    if line.trim() != "AA==" {
                        Some(line.trim())
                    } else {
                        None
                    }
                })
                .filter_map(|line| Tickers::try_from(line).ok())
                .for_each(|tickers| {
                    // dbg!("{:#?}", &tickers);
                    // print_tickers(&tickers);
                    // pretty_print_tickers(&tickers);
                    print_tickers(&tickers);
                    // write_json_file(&json_file, &tickers).ok();
                    // write_bin_file(&mut bin_file, &tickers).ok();
                    // tickers_array.lock().unwrap().push(tickers);
                });
            // buf_array = next_buf_array;
            buf_array.clear();
        }
        println!("nol: {}", nol);
        // println!(
        //     "tickers_array_len : {}",
        //     tickers_array.lock().unwrap().len()
        // );
        // for tickers in tickers_array.lock().unwrap().iter() {
        //     write_json_file(&json_file, tickers)?;
        //     write_bin_file(&mut bin_file, tickers)?;
        // }
        Ok(())
    }
}
