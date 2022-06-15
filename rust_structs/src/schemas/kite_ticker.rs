use super::common::*;
use crate::utils::*;
// use bincode::{Decode, Encode};
use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ltp {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ltpc {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
    pub close_price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexQuote {
    pub tradable: bool,
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub last_price: f64,
    pub ohlc: TickerOhlc,
    pub change: f64,
    pub price_change: PriceChange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexFull {
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

impl Default for IndexFull {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullExtendedDepth {
    pub mode: TickerStreamingMode,
    pub instrument_token: i32,
    pub depth: MarketDepth,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickerData {
    Ltp(Ltp),
    Ltpc(Ltpc),
    IndexQuote(IndexQuote),
    IndexFull(IndexFull),
    Quote(Quote),
    Full(Full),
    CompactFull(CompactFull),
    FullExtendedDepth(FullExtendedDepth),
}

impl Default for TickerData {
    fn default() -> Self {
        Self::Ltp(Default::default())
    }
}

// convertPrice converts prices of stocks from paise to rupees
// with varying decimals based on the segment.
fn convert_price(segment: i32, price: i32) -> f64 {
    let divisor_val: i32 = match segment {
        segment if segment == ExchangeMap::Cds as i32 => 10_000_000_i32,
        segment if segment == ExchangeMap::Bcd as i32 => 10_000_i32,
        _ => 100_i32,
    };
    price as f64 / divisor_val as f64
}

fn calculate_change(last_price: f64, open_price: f64, close_price: f64) -> PriceChange {
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

#[cfg(test)]
mod tests {
    use super::*;
    // use chrono::NaiveDate;
    use crate::utils::read_lines;
    // use byteorder::{BigEndian, ReadBytesExt};
    use chrono::{FixedOffset, TimeZone};
    use eio::ReadExt;
    use std::{
        fs::{File, OpenOptions},
        io::{Cursor, Seek, SeekFrom},
    };

    #[test]
    fn test_kite_ticker_raw() -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(lines) = read_lines("./custom_mock_files/ws.packet") {
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("kite_ticker.json")?;
            for line in lines {
                if let Ok(line) = line {
                    // let data = b"AAQAuACfOQIAAABkAAAASwAAARwByROfABZ+NAAFYmsAAAG9AAAB9AAAAFoAAAPUYqGllwBC8qwATzXuADU/QGKhpZgAAGTIAAAAZAAkAAAAAJTtAAAAXwA8AAAAADZlAAAAWgAoAAAAABX5AAAAVQARAAAAADGcAAAAUAAZAAAAABydAAAAaQAoAAAAAFHWAAAAbgA5AAAAABftAAAAcwAWAAAAABnhAAAAeAAVAAAAAA+5AAAAfQAQAAAADAAAAQkAU8ibAFPCYQAMADcfAQAAH3cAAB9FAAgAA/gJADUe/A==";
                    // let data = &base64::decode(data).unwrap()[..];
                    let data = &base64::decode(line.as_bytes()).unwrap()[..];
                    // println!("{}", data.len());
                    // println!("{:?}", &data);
                    if data.len() >= 2 {
                        let mut reader = Cursor::new(data);
                        // let number_of_packets = reader.read_i16::<BigEndian>().unwrap();
                        let number_of_packets: i16 = reader.read_be().unwrap();
                        // println!("number_of_packets => {number_of_packets}");
                        let mut ticker_data: Vec<TickerData> = Vec::new();
                        let mut packet_length: i16;
                        let mut j: u64 = 2;
                        for _ in 0..number_of_packets {
                            packet_length = reader.read_be().unwrap();
                            let instrument_token: i32 = reader.read_be().unwrap();
                            let segment = instrument_token & 0xFF;
                            let is_index: bool = segment == ExchangeMap::Indices as i32;
                            let tradable: bool = !is_index;
                            match packet_length {
                                // LTP / LTPC
                                8 | 12 => {
                                    let ltp_data = Ltp {
                                        tradable,
                                        mode: TickerStreamingMode::Ltp,
                                        instrument_token,
                                        last_price: convert_price(
                                            segment,
                                            reader.read_be().unwrap(),
                                        ),
                                    };
                                    if packet_length == 12 {
                                        ticker_data.push(TickerData::Ltpc(Ltpc {
                                            tradable,
                                            mode: TickerStreamingMode::Ltpc,
                                            instrument_token,
                                            last_price: ltp_data.last_price,
                                            close_price: convert_price(
                                                segment,
                                                reader.read_be().unwrap(),
                                            ),
                                        }));
                                    } else {
                                        ticker_data.push(TickerData::Ltp(ltp_data));
                                    }
                                }

                                // Index quote/full
                                28 | 32 => {
                                    let mut index_quote_data = IndexQuote {
                                        tradable,
                                        mode: if packet_length == 28 {
                                            TickerStreamingMode::Quote
                                        } else {
                                            TickerStreamingMode::Full
                                        },
                                        instrument_token,
                                        last_price: convert_price(
                                            segment,
                                            reader.read_be().unwrap(),
                                        ),
                                        ohlc: TickerOhlc {
                                            high: convert_price(segment, reader.read_be().unwrap()),
                                            low: convert_price(segment, reader.read_be().unwrap()),
                                            open: convert_price(segment, reader.read_be().unwrap()),
                                            close: convert_price(
                                                segment,
                                                reader.read_be().unwrap(),
                                            ),
                                        },
                                        change: reader.read_be::<i32>().unwrap() as f64,
                                        price_change: PriceChange::default(),
                                    };

                                    index_quote_data.price_change = calculate_change(
                                        index_quote_data.last_price,
                                        index_quote_data.ohlc.open,
                                        index_quote_data.ohlc.close,
                                    );

                                    if packet_length == 32 {
                                        ticker_data.push(TickerData::IndexFull(IndexFull {
                                            tradable,
                                            mode: TickerStreamingMode::Full,
                                            instrument_token,
                                            last_price: index_quote_data.last_price,
                                            ohlc: TickerOhlc {
                                                high: index_quote_data.ohlc.high,
                                                low: index_quote_data.ohlc.low,
                                                open: index_quote_data.ohlc.open,
                                                close: index_quote_data.ohlc.close,
                                            },
                                            change: index_quote_data.change,
                                            price_change: index_quote_data.price_change,
                                            exchange_timestamp: FixedOffset::east(19800).timestamp(
                                                reader.read_be::<i32>().unwrap() as i64,
                                                0,
                                            ),
                                        }));
                                    } else {
                                        ticker_data.push(TickerData::IndexQuote(index_quote_data));
                                    }
                                }

                                // Quote/Full
                                44 | 164 | 184 => {
                                    let mut quote_data = Quote {
                                        tradable,
                                        mode: if packet_length == 44 {
                                            TickerStreamingMode::Quote
                                        } else {
                                            TickerStreamingMode::Full
                                        },
                                        instrument_token,
                                        last_price: convert_price(
                                            segment,
                                            reader.read_be().unwrap(),
                                        ),
                                        last_traded_quantity: reader.read_be().unwrap(),
                                        average_traded_price: convert_price(
                                            segment,
                                            reader.read_be().unwrap(),
                                        ),
                                        volume_traded: reader.read_be().unwrap(),
                                        total_buy_quantity: reader.read_be().unwrap(),
                                        total_sell_quantity: reader.read_be().unwrap(),
                                        ohlc: TickerOhlc {
                                            open: convert_price(segment, reader.read_be().unwrap()),
                                            high: convert_price(segment, reader.read_be().unwrap()),
                                            low: convert_price(segment, reader.read_be().unwrap()),
                                            close: convert_price(
                                                segment,
                                                reader.read_be().unwrap(),
                                            ),
                                        },
                                        price_change: PriceChange::default(),
                                    };

                                    quote_data.price_change = calculate_change(
                                        quote_data.last_price,
                                        quote_data.ohlc.open,
                                        quote_data.ohlc.close,
                                    );

                                    if packet_length == 164 {
                                        let mut compact_full_data = CompactFull {
                                            tradable,
                                            mode: TickerStreamingMode::Full,
                                            instrument_token,
                                            last_price: quote_data.last_price,
                                            last_traded_quantity: quote_data.last_traded_quantity,
                                            average_traded_price: quote_data.average_traded_price,
                                            volume_traded: quote_data.volume_traded,
                                            total_buy_quantity: quote_data.total_buy_quantity,
                                            total_sell_quantity: quote_data.total_sell_quantity,
                                            ohlc: TickerOhlc {
                                                open: quote_data.ohlc.open,
                                                high: quote_data.ohlc.high,
                                                low: quote_data.ohlc.low,
                                                close: quote_data.ohlc.close,
                                            },
                                            price_change: quote_data.price_change,
                                            depth: MarketDepth {
                                                buy: Vec::with_capacity(5),
                                                sell: Vec::with_capacity(5),
                                            },
                                        };
                                        for index in 0..10 {
                                            let depth_data = MarketDepthData {
                                                quantity: reader.read_be().unwrap(),
                                                price: convert_price(
                                                    segment,
                                                    reader.read_be().unwrap(),
                                                ),
                                                orders: reader.read_be::<i16>().unwrap() as i32,
                                            };
                                            if index < 5 {
                                                compact_full_data.depth.buy.push(depth_data);
                                            } else {
                                                compact_full_data.depth.sell.push(depth_data);
                                            }
                                            // Dont care 2 bytes padding
                                            reader.read_be::<i16>().unwrap();
                                        }
                                        ticker_data
                                            .push(TickerData::CompactFull(compact_full_data));
                                    } else if packet_length == 184 {
                                        let mut full_data = Full {
                                            tradable,
                                            mode: TickerStreamingMode::Full,
                                            instrument_token,
                                            last_price: quote_data.last_price,
                                            last_traded_quantity: quote_data.last_traded_quantity,
                                            average_traded_price: quote_data.average_traded_price,
                                            volume_traded: quote_data.volume_traded,
                                            total_buy_quantity: quote_data.total_buy_quantity,
                                            total_sell_quantity: quote_data.total_sell_quantity,
                                            ohlc: TickerOhlc {
                                                open: quote_data.ohlc.open,
                                                high: quote_data.ohlc.high,
                                                low: quote_data.ohlc.low,
                                                close: quote_data.ohlc.close,
                                            },
                                            price_change: quote_data.price_change,
                                            last_trade_time: FixedOffset::east(19800).timestamp(
                                                reader.read_be::<i32>().unwrap() as i64,
                                                0,
                                            ),
                                            oi: reader.read_be().unwrap(),
                                            oi_day_high: reader.read_be().unwrap(),
                                            oi_day_low: reader.read_be().unwrap(),
                                            exchange_timestamp: FixedOffset::east(19800).timestamp(
                                                reader.read_be::<i32>().unwrap() as i64,
                                                0,
                                            ),
                                            depth: MarketDepth {
                                                buy: Vec::with_capacity(5),
                                                sell: Vec::with_capacity(5),
                                            },
                                        };
                                        for index in 0..10 {
                                            let depth_data = MarketDepthData {
                                                quantity: reader.read_be().unwrap(),
                                                price: convert_price(
                                                    segment,
                                                    reader.read_be().unwrap(),
                                                ),
                                                orders: reader.read_be::<i16>().unwrap() as i32,
                                            };
                                            if index < 5 {
                                                full_data.depth.buy.push(depth_data);
                                            } else {
                                                full_data.depth.sell.push(depth_data);
                                            }
                                            // Dont care 2 bytes padding
                                            reader.read_be::<i16>().unwrap();
                                        }
                                        ticker_data.push(TickerData::Full(full_data));
                                    } else {
                                        ticker_data.push(TickerData::Quote(quote_data));
                                    }
                                }
                                // Full with extended 20 depth
                                492 => {
                                    let mut full_extended_depth_data = FullExtendedDepth {
                                        mode: TickerStreamingMode::Full,
                                        instrument_token,
                                        depth: MarketDepth {
                                            buy: Vec::with_capacity(20),
                                            sell: Vec::with_capacity(20),
                                        },
                                    };
                                    // Dont care 8 bytes padding
                                    // reader.read_i64::<BigEndian>().unwrap();
                                    println!("{}", reader.read_be::<i32>().unwrap());
                                    println!("{}", reader.read_be::<i32>().unwrap());
                                    // XXX We have already read 12 bytes now, Remaining 492-12/12 = 40
                                    for index in 0..40 {
                                        let depth_data = MarketDepthData {
                                            quantity: reader.read_be().unwrap(),
                                            price: convert_price(
                                                segment,
                                                reader.read_be().unwrap(),
                                            ),
                                            orders: reader.read_be().unwrap(),
                                        };
                                        if index < 20 {
                                            full_extended_depth_data.depth.buy.push(depth_data);
                                        } else {
                                            full_extended_depth_data.depth.sell.push(depth_data);
                                        }
                                    }
                                    ticker_data.push(TickerData::FullExtendedDepth(
                                        full_extended_depth_data,
                                    ));
                                }
                                _ => {
                                    println!(
                                        "Error => undefined packet length received: {}",
                                        packet_length
                                    )
                                }
                            }
                            // Place reader in the position after the packet length
                            j += 2 + packet_length as u64;
                            reader.seek(SeekFrom::Start(j))?;
                            // reader.seek(SeekFrom::Start(j + 2 + packet_length as u64))?;
                            // j = j + 2 + packet_length as u64;
                        }
                        // println!("{}", serde_json::to_string_pretty(&tick_data).unwrap());
                        println!("{}", serde_json::to_string_pretty(&ticker_data).unwrap());
                        // println!("{:#?}", &ticker_data);
                        serde_json::to_writer_pretty(&file, &ticker_data)?;
                    }
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_kite_ticker_raw_single() -> Result<(), Box<dyn std::error::Error>> {
        let data = b"AAQACAAD+AkANR9gALgAnzoCAAHViAAAABkAAdQLAANHjQAEGD4AAHxRAAHFAgACDWQAAY7ZAAGWaGKhpZkAA37sAAOgtgADTQVioaWbAAAAGQAB1ZIAAQAAAAAAfQAB1Y0AAQAAAAAAGQAB1YgAAQAAAAAAMgAB1YMAAQAAAAAASwAB1X4AAgAAAAAAGQAB1q8AAQAAAAAAGQAB1tcAAQAAAAAAMgAB1uEAAQAAAAAAyAAB1uYAAgAAAAAAGQAB1usAAQAAALgAnzkCAAAAaQAAABkAAAEcAckZrQAWhBAABWLoAAABvQAAAfQAAABaAAAD1GKhpZwAQvKsAE817gA1P0BioaWcAABhwQAAAGQAJgAAAACQ7AAAAF8AOQAAAAA2MwAAAFoAJwAAAAAbJgAAAFUAFwAAAAA0PwAAAFAAHAAAAAAhAgAAAGkALgAAAABOhAAAAG4AOQAAAAAZlgAAAHMAGgAAAAAZ4QAAAHgAFQAAAAAPuQAAAH0AEAAAAAwANx8BAAAffAAAH0U=";
        let data = &base64::decode(data).unwrap()[..];
        // println!("{}", data.len());
        // println!("{:?}", &data);
        if data.len() >= 2 {
            let mut reader = Cursor::new(data);
            // let number_of_packets = reader.read_i16::<BigEndian>().unwrap();
            let number_of_packets: i16 = reader.read_be().unwrap();
            // println!("number_of_packets => {number_of_packets}");
            let mut ticker_data: Vec<TickerData> = Vec::new();
            let mut packet_length: i16;
            let mut j: u64 = 2;
            for _ in 0..number_of_packets {
                packet_length = reader.read_be().unwrap();
                let instrument_token: i32 = reader.read_be().unwrap();
                let segment = instrument_token & 0xFF;
                let is_index: bool = segment == ExchangeMap::Indices as i32;
                let tradable: bool = !is_index;
                match packet_length {
                    // LTP / LTPC
                    8 | 12 => {
                        let ltp_data = Ltp {
                            tradable,
                            mode: TickerStreamingMode::Ltp,
                            instrument_token,
                            last_price: convert_price(segment, reader.read_be().unwrap()),
                        };
                        if packet_length == 12 {
                            ticker_data.push(TickerData::Ltpc(Ltpc {
                                tradable,
                                mode: TickerStreamingMode::Ltpc,
                                instrument_token,
                                last_price: ltp_data.last_price,
                                close_price: convert_price(segment, reader.read_be().unwrap()),
                            }));
                        } else {
                            ticker_data.push(TickerData::Ltp(ltp_data));
                        }
                    }

                    // Index quote/full
                    28 | 32 => {
                        let mut index_quote_data = IndexQuote {
                            tradable,
                            mode: if packet_length == 28 {
                                TickerStreamingMode::Quote
                            } else {
                                TickerStreamingMode::Full
                            },
                            instrument_token,
                            last_price: convert_price(segment, reader.read_be().unwrap()),
                            ohlc: TickerOhlc {
                                high: convert_price(segment, reader.read_be().unwrap()),
                                low: convert_price(segment, reader.read_be().unwrap()),
                                open: convert_price(segment, reader.read_be().unwrap()),
                                close: convert_price(segment, reader.read_be().unwrap()),
                            },
                            change: reader.read_be::<i32>().unwrap() as f64,
                            price_change: PriceChange::default(),
                        };

                        index_quote_data.price_change = calculate_change(
                            index_quote_data.last_price,
                            index_quote_data.ohlc.open,
                            index_quote_data.ohlc.close,
                        );

                        if packet_length == 32 {
                            ticker_data.push(TickerData::IndexFull(IndexFull {
                                tradable,
                                mode: TickerStreamingMode::Full,
                                instrument_token,
                                last_price: index_quote_data.last_price,
                                ohlc: TickerOhlc {
                                    high: index_quote_data.ohlc.high,
                                    low: index_quote_data.ohlc.low,
                                    open: index_quote_data.ohlc.open,
                                    close: index_quote_data.ohlc.close,
                                },
                                change: index_quote_data.change,
                                price_change: index_quote_data.price_change,
                                exchange_timestamp: FixedOffset::east(19800)
                                    .timestamp(reader.read_be::<i32>().unwrap() as i64, 0),
                            }));
                        } else {
                            ticker_data.push(TickerData::IndexQuote(index_quote_data));
                        }
                    }

                    // Quote/Full
                    44 | 164 | 184 => {
                        let mut quote_data = Quote {
                            tradable,
                            mode: if packet_length == 44 {
                                TickerStreamingMode::Quote
                            } else {
                                TickerStreamingMode::Full
                            },
                            instrument_token,
                            last_price: convert_price(segment, reader.read_be().unwrap()),
                            last_traded_quantity: reader.read_be().unwrap(),
                            average_traded_price: convert_price(segment, reader.read_be().unwrap()),
                            volume_traded: reader.read_be().unwrap(),
                            total_buy_quantity: reader.read_be().unwrap(),
                            total_sell_quantity: reader.read_be().unwrap(),
                            ohlc: TickerOhlc {
                                open: convert_price(segment, reader.read_be().unwrap()),
                                high: convert_price(segment, reader.read_be().unwrap()),
                                low: convert_price(segment, reader.read_be().unwrap()),
                                close: convert_price(segment, reader.read_be().unwrap()),
                            },
                            price_change: PriceChange::default(),
                        };

                        quote_data.price_change = calculate_change(
                            quote_data.last_price,
                            quote_data.ohlc.open,
                            quote_data.ohlc.close,
                        );

                        if packet_length == 164 {
                            let mut compact_full_data = CompactFull {
                                tradable,
                                mode: TickerStreamingMode::Full,
                                instrument_token,
                                last_price: quote_data.last_price,
                                last_traded_quantity: quote_data.last_traded_quantity,
                                average_traded_price: quote_data.average_traded_price,
                                volume_traded: quote_data.volume_traded,
                                total_buy_quantity: quote_data.total_buy_quantity,
                                total_sell_quantity: quote_data.total_sell_quantity,
                                ohlc: TickerOhlc {
                                    open: quote_data.ohlc.open,
                                    high: quote_data.ohlc.high,
                                    low: quote_data.ohlc.low,
                                    close: quote_data.ohlc.close,
                                },
                                price_change: quote_data.price_change,
                                depth: MarketDepth {
                                    buy: Vec::with_capacity(5),
                                    sell: Vec::with_capacity(5),
                                },
                            };
                            for index in 0..10 {
                                let depth_data = MarketDepthData {
                                    quantity: reader.read_be().unwrap(),
                                    price: convert_price(segment, reader.read_be().unwrap()),
                                    orders: reader.read_be::<i16>().unwrap() as i32,
                                };
                                if index < 5 {
                                    compact_full_data.depth.buy.push(depth_data);
                                } else {
                                    compact_full_data.depth.sell.push(depth_data);
                                }
                                // Dont care 2 bytes padding
                                reader.read_be::<i16>().unwrap();
                            }
                            ticker_data.push(TickerData::CompactFull(compact_full_data));
                        } else if packet_length == 184 {
                            let mut full_data = Full {
                                tradable,
                                mode: TickerStreamingMode::Full,
                                instrument_token,
                                last_price: quote_data.last_price,
                                last_traded_quantity: quote_data.last_traded_quantity,
                                average_traded_price: quote_data.average_traded_price,
                                volume_traded: quote_data.volume_traded,
                                total_buy_quantity: quote_data.total_buy_quantity,
                                total_sell_quantity: quote_data.total_sell_quantity,
                                ohlc: TickerOhlc {
                                    open: quote_data.ohlc.open,
                                    high: quote_data.ohlc.high,
                                    low: quote_data.ohlc.low,
                                    close: quote_data.ohlc.close,
                                },
                                price_change: quote_data.price_change,
                                last_trade_time: FixedOffset::east(19800)
                                    .timestamp(reader.read_be::<i32>().unwrap() as i64, 0),
                                oi: reader.read_be().unwrap(),
                                oi_day_high: reader.read_be().unwrap(),
                                oi_day_low: reader.read_be().unwrap(),
                                exchange_timestamp: FixedOffset::east(19800)
                                    .timestamp(reader.read_be::<i32>().unwrap() as i64, 0),
                                depth: MarketDepth {
                                    buy: Vec::with_capacity(5),
                                    sell: Vec::with_capacity(5),
                                },
                            };
                            for index in 0..10 {
                                let depth_data = MarketDepthData {
                                    quantity: reader.read_be().unwrap(),
                                    price: convert_price(segment, reader.read_be().unwrap()),
                                    orders: reader.read_be::<i16>().unwrap() as i32,
                                };
                                if index < 5 {
                                    full_data.depth.buy.push(depth_data);
                                } else {
                                    full_data.depth.sell.push(depth_data);
                                }
                                // Dont care 2 bytes padding
                                reader.read_be::<i16>().unwrap();
                            }
                            ticker_data.push(TickerData::Full(full_data));
                        } else {
                            ticker_data.push(TickerData::Quote(quote_data));
                        }
                    }
                    // Full with extended 20 depth
                    492 => {
                        let mut full_extended_depth_data = FullExtendedDepth {
                            mode: TickerStreamingMode::Full,
                            instrument_token,
                            depth: MarketDepth {
                                buy: Vec::with_capacity(20),
                                sell: Vec::with_capacity(20),
                            },
                        };
                        // Dont care 8 bytes padding
                        // reader.read_i64::<BigEndian>().unwrap();
                        println!("{}", reader.read_be::<i32>().unwrap());
                        println!("{}", reader.read_be::<i32>().unwrap());
                        // XXX We have already read 12 bytes now, Remaining 492-12/12 = 40
                        for index in 0..40 {
                            let depth_data = MarketDepthData {
                                quantity: reader.read_be().unwrap(),
                                price: convert_price(segment, reader.read_be().unwrap()),
                                orders: reader.read_be().unwrap(),
                            };
                            if index < 20 {
                                full_extended_depth_data.depth.buy.push(depth_data);
                            } else {
                                full_extended_depth_data.depth.sell.push(depth_data);
                            }
                        }
                        ticker_data.push(TickerData::FullExtendedDepth(full_extended_depth_data));
                    }
                    _ => {
                        println!(
                            "Error => undefined packet length received: {}",
                            packet_length
                        )
                    }
                }
                // Place reader in the position after the packet length
                j += 2 + packet_length as u64;
                reader.seek(SeekFrom::Start(j))?;
                // reader.seek(SeekFrom::Start(j + 2 + packet_length as u64))?;
                // j = j + 2 + packet_length as u64;
            }
            // println!("{}", serde_json::to_string_pretty(&tick_data).unwrap());
            println!("{}", serde_json::to_string_pretty(&ticker_data).unwrap());
            // println!("{:#?}", &ticker_data);
            serde_json::to_writer_pretty(&File::create("kite_ticker.json")?, &ticker_data)?;
        }
        Ok(())
    }
}


// fn main() {
//     let array: [u8; 4] = [0, 1, 2, 3];

//     let [chunk_0, chunk_1]: [[u8; 2]; 2] =
//         unsafe { std::mem::transmute::<[u8; 4], [[u8; 2]; 2]>(array) };

//     assert_eq!([0, 1], chunk_0);
//     assert_eq!([2, 3], chunk_1);
// }
