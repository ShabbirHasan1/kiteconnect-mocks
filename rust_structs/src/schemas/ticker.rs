use super::common::*;
use super::ticker_structs::*;
use crate::utils::*;
// use bincode::{Decode, Encode};
use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

// Refer https://kite.trade/docs/connect/v3/websocket/

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct LtpBin {
    pub instrument_token: [u8; 4],
    pub last_price: [u8; 4],
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct LtpcBin {
    pub instrument_token: [u8; 4],
    pub last_price: [u8; 4],
    pub close_price: [u8; 4],
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub struct QuoteIndexBin {
    pub instrument_token: [u8; 4],
    pub last_price: [u8; 4],
    pub ohlc: IndexTickerOhlc,
    pub change: [u8; 4],
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct FullIndexBin {
    pub instrument_token: [u8; 4],
    pub last_price: [u8; 4],
    pub ohlc: IndexTickerOhlc,
    pub change: [u8; 4],
    pub exchange_timestamp: [u8; 4],
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct IndexTickerOhlc {
    pub high: [u8; 4],
    pub low: [u8; 4],
    pub open: [u8; 4],
    pub close: [u8; 4],
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct TickerOhlc {
    pub open: [u8; 4],
    pub high: [u8; 4],
    pub low: [u8; 4],
    pub close: [u8; 4],
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct QuoteBin {
    pub instrument_token: [u8; 4],
    pub last_price: [u8; 4],
    pub last_traded_quantity: [u8; 4],
    pub average_traded_price: [u8; 4],
    pub volume_traded: [u8; 4],
    pub total_buy_quantity: [u8; 4],
    pub total_sell_quantity: [u8; 4],
    pub ohlc: TickerOhlc,
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct FullBin {
    pub instrument_token: [u8; 4],
    pub last_price: [u8; 4],
    pub last_traded_quantity: [u8; 4],
    pub average_traded_price: [u8; 4],
    pub volume_traded: [u8; 4],
    pub total_buy_quantity: [u8; 4],
    pub total_sell_quantity: [u8; 4],
    pub ohlc: TickerOhlc,
    pub last_trade_time: [u8; 4],
    pub oi: [u8; 4],
    pub oi_day_low: [u8; 4],
    pub oi_day_high: [u8; 4],
    pub exchange_timestamp: [u8; 4],
    pub depth: MarketDepthBin,
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct CompactFullBin {
    pub instrument_token: [u8; 4],
    pub last_price: [u8; 4],
    pub last_traded_quantity: [u8; 4],
    pub average_traded_price: [u8; 4],
    pub volume_traded: [u8; 4],
    pub total_buy_quantity: [u8; 4],
    pub total_sell_quantity: [u8; 4],
    pub ohlc: TickerOhlc,
    pub depth: MarketDepthBin,
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct ExtendedDepthBin {
    pub instrument_token: [u8; 4],
    pub padding: i64,
    pub depth: ExtendedMarketDepthBin,
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct MarketDepthBin {
    pub buy: [MarketDepthDataBin; 5],
    pub sell: [MarketDepthDataBin; 5],
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct MarketDepthDataBin {
    pub quantity: [u8; 4],
    pub price: [u8; 4],
    pub orders: [u8; 2],
    pub padding: [u8; 2],
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct ExtendedMarketDepthBin {
    pub buy: [ExtendedMarketDepthDataBin; 20],
    pub sell: [ExtendedMarketDepthDataBin; 20],
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct ExtendedMarketDepthDataBin {
    pub quantity: [u8; 4],
    pub price: [u8; 4],
    pub orders: [u8; 4],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickerData {
    LtpBin(LtpBin),
    LtpcBin(LtpcBin),
    QuoteIndexBin(QuoteIndexBin),
    FullIndexBin(FullIndexBin),
    QuoteBin(QuoteBin),
    FullBin(FullBin),
    CompactFullBin(CompactFullBin),
    FullBinExtendedDepthBin(ExtendedDepthBin),
}

impl Default for TickerData {
    fn default() -> Self {
        Self::LtpBin(Default::default())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryKiteTickerMessage {
    pub packet_size: i16,
    pub ticker_data: Vec<BinaryKiteTickerMessageDecoder>,
}

#[derive(Default, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct BinaryKiteTickerMessageDecoder {
    pub packet_length: i16,
    pub packet_schema_type_data: PacketSchemaTypeData,
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum PacketSchemaType {
    LTP,
    LTPC,
    QUOTEINDEX,
    FULLINDEX,
    QUOTE,
    FULL,
    COMPACTFULL,
    EXTENDEDDEPTH,
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PacketSchemaTypeData {
    LTP(LtpBin),
    LTPC(LtpcBin),
    QUOTEINDEX(QuoteIndexBin),
    FULLINDEX(FullIndexBin),
    QUOTE(QuoteBin),
    FULL(FullBin),
    COMPACTFULL(CompactFullBin),
    EXTENDEDDEPTH(ExtendedDepthBin),
}

impl PacketSchemaType {
    fn default() -> Self {
        Self::LTP
    }

    fn from(num: i16) -> Option<Self> {
        match num {
            8 => Some(PacketSchemaType::LTP),
            12 => Some(PacketSchemaType::LTPC),
            28 => Some(PacketSchemaType::QUOTEINDEX),
            32 => Some(PacketSchemaType::FULLINDEX),
            44 => Some(PacketSchemaType::QUOTE),
            164 => Some(PacketSchemaType::COMPACTFULL),
            184 => Some(PacketSchemaType::FULL),
            492 => Some(PacketSchemaType::EXTENDEDDEPTH),
            _ => None,
        }
    }
}

impl Default for PacketSchemaTypeData {
    fn default() -> Self {
        Self::LTP(Default::default())
    }
}

impl PacketSchemaTypeData {
    fn from(packet_schema_type: PacketSchemaType, packet_data: &[u8]) -> Self {
        match packet_schema_type {
            PacketSchemaType::LTP => {
                assert_eq!(
                    std::mem::size_of::<LtpBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let mut packet_data_buf = [0_u8; std::mem::size_of::<LtpBin>()];
                packet_data_buf.copy_from_slice(packet_data);
                assert_eq!(
                    packet_data_buf.len(),
                    8,
                    "Didn't read all the bytes into memory"
                );
                Self::LTP(unsafe {
                    std::mem::transmute(packet_data_buf)
                    // std::mem::transmute(packet_data.as_ptr() as *const LtpBin)
                })
            }
            PacketSchemaType::LTPC => {
                assert_eq!(
                    std::mem::size_of::<LtpcBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let mut packet_data_buf = [0_u8; std::mem::size_of::<LtpcBin>()];
                packet_data_buf.copy_from_slice(packet_data);
                assert_eq!(
                    packet_data_buf.len(),
                    12,
                    "Didn't read all the bytes into memory"
                );
                Self::LTPC(unsafe {
                    std::mem::transmute(packet_data_buf)
                    // std::mem::transmute(packet_data.as_ptr() as *const LtpcBin)
                })
            }
            PacketSchemaType::QUOTEINDEX => {
                assert_eq!(
                    std::mem::size_of::<QuoteIndexBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let mut packet_data_buf = [0_u8; std::mem::size_of::<QuoteIndexBin>()];
                packet_data_buf.copy_from_slice(packet_data);
                assert_eq!(
                    packet_data_buf.len(),
                    28,
                    "Didn't read all the bytes into memory"
                );
                Self::QUOTEINDEX(unsafe {
                    std::mem::transmute(packet_data_buf)
                    // std::mem::transmute(packet_data.as_ptr() as *const QuoteIndexBin)
                })
            }
            PacketSchemaType::FULLINDEX => {
                assert_eq!(
                    std::mem::size_of::<FullIndexBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let mut packet_data_buf = [0_u8; std::mem::size_of::<FullIndexBin>()];
                packet_data_buf.copy_from_slice(packet_data);
                assert_eq!(
                    packet_data_buf.len(),
                    32,
                    "Didn't read all the bytes into memory"
                );
                Self::FULLINDEX(unsafe {
                    std::mem::transmute(packet_data_buf)
                    // std::mem::transmute(packet_data.as_ptr() as *const FullIndexBin)
                })
            }
            PacketSchemaType::QUOTE => {
                assert_eq!(
                    std::mem::size_of::<QuoteBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let mut packet_data_buf = [0_u8; std::mem::size_of::<QuoteBin>()];
                packet_data_buf.copy_from_slice(packet_data);
                assert_eq!(
                    packet_data_buf.len(),
                    44,
                    "Didn't read all the bytes into memory"
                );
                Self::QUOTE(unsafe {
                    std::mem::transmute(packet_data_buf)
                    // std::mem::transmute(packet_data.as_ptr() as *const QuoteBin)
                })
            }
            PacketSchemaType::FULL => {
                assert_eq!(
                    std::mem::size_of::<FullBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let mut packet_data_buf = [0_u8; std::mem::size_of::<FullBin>()];
                packet_data_buf.copy_from_slice(packet_data);
                assert_eq!(
                    packet_data_buf.len(),
                    184,
                    "Didn't read all the bytes into memory"
                );
                Self::FULL(unsafe {
                    std::mem::transmute(packet_data_buf)
                    // std::mem::transmute(packet_data.as_ptr() as *const FullBin)
                })
            }
            PacketSchemaType::COMPACTFULL => {
                assert_eq!(
                    std::mem::size_of::<CompactFullBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let mut packet_data_buf = [0_u8; std::mem::size_of::<CompactFullBin>()];
                packet_data_buf.copy_from_slice(packet_data);
                assert_eq!(
                    packet_data_buf.len(),
                    164,
                    "Didn't read all the bytes into memory"
                );
                Self::COMPACTFULL(unsafe {
                    std::mem::transmute(packet_data_buf)
                    // std::mem::transmute(packet_data.as_ptr() as *const CompactFullBin)
                })
            }
            PacketSchemaType::EXTENDEDDEPTH => {
                assert_eq!(
                    std::mem::size_of::<ExtendedDepthBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let mut packet_data_buf = [0_u8; std::mem::size_of::<ExtendedDepthBin>()];
                packet_data_buf.copy_from_slice(packet_data);
                assert_eq!(
                    packet_data_buf.len(),
                    492,
                    "Didn't read all the bytes into memory"
                );
                Self::EXTENDEDDEPTH(unsafe {
                    std::mem::transmute(packet_data_buf)
                    // std::mem::transmute(packet_data.as_ptr() as *const ExtendedDepthBin)
                })
            }
            _ => panic!("Unknown packet schema type"),
        }
    }

    fn from1(packet_schema_type: PacketSchemaType, packet_data: &[u8]) -> Self {
        match packet_schema_type {
            PacketSchemaType::LTP => {
                assert_eq!(
                    std::mem::size_of::<LtpBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let (head, body, _tail) = unsafe { packet_data.align_to::<LtpBin>() };
                assert!(head.is_empty(), "Data was not aligned");
                let ticker_message = &body[0];
                Self::LTP(*ticker_message)
            }
            PacketSchemaType::LTPC => {
                assert_eq!(
                    std::mem::size_of::<LtpcBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let (head, body, _tail) = unsafe { packet_data.align_to::<LtpcBin>() };
                assert!(head.is_empty(), "Data was not aligned");
                let ticker_message = &body[0];
                Self::LTPC(*ticker_message)
            }
            PacketSchemaType::QUOTEINDEX => {
                assert_eq!(
                    std::mem::size_of::<QuoteIndexBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let (head, body, _tail) = unsafe { packet_data.align_to::<QuoteIndexBin>() };
                assert!(head.is_empty(), "Data was not aligned");
                let ticker_message = &body[0];
                Self::QUOTEINDEX(*ticker_message)
            }
            PacketSchemaType::FULLINDEX => {
                assert_eq!(
                    std::mem::size_of::<FullIndexBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let (head, body, _tail) = unsafe { packet_data.align_to::<FullIndexBin>() };
                assert!(head.is_empty(), "Data was not aligned");
                let ticker_message = &body[0];
                Self::FULLINDEX(*ticker_message)
            }
            PacketSchemaType::QUOTE => {
                assert_eq!(
                    std::mem::size_of::<QuoteBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let (head, body, _tail) = unsafe { packet_data.align_to::<QuoteBin>() };
                assert!(head.is_empty(), "Data was not aligned");
                let ticker_message = &body[0];
                Self::QUOTE(*ticker_message)
            }
            PacketSchemaType::FULL => {
                assert_eq!(
                    std::mem::size_of::<FullBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let (head, body, _tail) = unsafe { packet_data.align_to::<FullBin>() };
                assert!(head.is_empty(), "Data was not aligned");
                let ticker_message = &body[0];
                Self::FULL(*ticker_message)
            }
            PacketSchemaType::COMPACTFULL => {
                assert_eq!(
                    std::mem::size_of::<CompactFullBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let (head, body, _tail) = unsafe { packet_data.align_to::<CompactFullBin>() };
                assert!(head.is_empty(), "Data was not aligned");
                let ticker_message = &body[0];
                Self::COMPACTFULL(*ticker_message)
            }
            PacketSchemaType::EXTENDEDDEPTH => {
                assert_eq!(
                    std::mem::size_of::<ExtendedDepthBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let (head, body, _tail) = unsafe { packet_data.align_to::<ExtendedDepthBin>() };
                assert!(head.is_empty(), "Data was not aligned");
                let ticker_message = &body[0];
                Self::EXTENDEDDEPTH(*ticker_message)
            }
            _ => panic!("Unknown packet schema type"),
        }
    }

    fn from2(packet_schema_type: PacketSchemaType, packet_data: &[u8]) -> Self {
        match packet_schema_type {
            PacketSchemaType::LTP => {
                assert_eq!(
                    std::mem::size_of::<LtpBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let try_struct_decode = packet_data.as_ptr() as *mut LtpBin;
                let struct_decoded =
                    if std::mem::size_of::<LtpBin>() <= std::mem::size_of_val(packet_data) {
                        unsafe {
                            let ref intermediate_struct_decoded = *try_struct_decode;
                            Some(intermediate_struct_decoded)
                        }
                    } else {
                        None
                    };
                Self::LTP(*struct_decoded.unwrap())
            }
            PacketSchemaType::LTPC => {
                assert_eq!(
                    std::mem::size_of::<LtpcBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let try_struct_decode = packet_data.as_ptr() as *mut LtpcBin;
                let struct_decoded =
                    if std::mem::size_of::<LtpcBin>() <= std::mem::size_of_val(packet_data) {
                        unsafe {
                            let ref intermediate_struct_decoded = *try_struct_decode;
                            Some(intermediate_struct_decoded)
                        }
                    } else {
                        None
                    };
                Self::LTPC(*struct_decoded.unwrap())
            }
            PacketSchemaType::QUOTEINDEX => {
                assert_eq!(
                    std::mem::size_of::<QuoteIndexBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let try_struct_decode = packet_data.as_ptr() as *mut QuoteIndexBin;
                let struct_decoded =
                    if std::mem::size_of::<QuoteIndexBin>() <= std::mem::size_of_val(packet_data) {
                        unsafe {
                            let ref intermediate_struct_decoded = *try_struct_decode;
                            Some(intermediate_struct_decoded)
                        }
                    } else {
                        None
                    };
                Self::QUOTEINDEX(*struct_decoded.unwrap())
            }
            PacketSchemaType::FULLINDEX => {
                assert_eq!(
                    std::mem::size_of::<FullIndexBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let try_struct_decode = packet_data.as_ptr() as *mut FullIndexBin;
                let struct_decoded =
                    if std::mem::size_of::<FullIndexBin>() <= std::mem::size_of_val(packet_data) {
                        unsafe {
                            let ref intermediate_struct_decoded = *try_struct_decode;
                            Some(intermediate_struct_decoded)
                        }
                    } else {
                        None
                    };
                Self::FULLINDEX(*struct_decoded.unwrap())
            }
            PacketSchemaType::QUOTE => {
                assert_eq!(
                    std::mem::size_of::<QuoteBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let try_struct_decode = packet_data.as_ptr() as *mut QuoteBin;
                let struct_decoded =
                    if std::mem::size_of::<QuoteBin>() <= std::mem::size_of_val(packet_data) {
                        unsafe {
                            let ref intermediate_struct_decoded = *try_struct_decode;
                            Some(intermediate_struct_decoded)
                        }
                    } else {
                        None
                    };
                Self::QUOTE(*struct_decoded.unwrap())
            }
            PacketSchemaType::FULL => {
                assert_eq!(
                    std::mem::size_of::<FullBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let try_struct_decode = packet_data.as_ptr() as *mut FullBin;
                let struct_decoded =
                    if std::mem::size_of::<FullBin>() <= std::mem::size_of_val(packet_data) {
                        unsafe {
                            let ref intermediate_struct_decoded = *try_struct_decode;
                            Some(intermediate_struct_decoded)
                        }
                    } else {
                        None
                    };
                Self::FULL(*struct_decoded.unwrap())
            }
            PacketSchemaType::COMPACTFULL => {
                assert_eq!(
                    std::mem::size_of::<CompactFullBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let try_struct_decode = packet_data.as_ptr() as *mut CompactFullBin;
                let struct_decoded = if std::mem::size_of::<CompactFullBin>()
                    <= std::mem::size_of_val(packet_data)
                {
                    unsafe {
                        let ref intermediate_struct_decoded = *try_struct_decode;
                        Some(intermediate_struct_decoded)
                    }
                } else {
                    None
                };
                Self::COMPACTFULL(*struct_decoded.unwrap())
            }
            PacketSchemaType::EXTENDEDDEPTH => {
                assert_eq!(
                    std::mem::size_of::<ExtendedDepthBin>(),
                    std::mem::size_of_val(packet_data)
                );
                let try_struct_decode = packet_data.as_ptr() as *mut ExtendedDepthBin;
                let struct_decoded = if std::mem::size_of::<ExtendedDepthBin>()
                    <= std::mem::size_of_val(packet_data)
                {
                    unsafe {
                        let ref intermediate_struct_decoded = *try_struct_decode;
                        Some(intermediate_struct_decoded)
                    }
                } else {
                    None
                };
                Self::EXTENDEDDEPTH(*struct_decoded.unwrap())
            }
            _ => panic!("Unknown packet schema type"),
        }
    }
}

impl BinaryKiteTickerMessage {
    // fn from(input: Vec<u8>) -> BinaryKiteTickerMessage {
    fn from(input: Vec<u8>) -> Vec<TickerSchemaTypeData> {
        let (int_bytes, rest) = input.split_at(std::mem::size_of::<i16>());
        // let mut ticker_data: Vec<BinaryKiteTickerMessageDecoder> = Vec::new();
        let mut ticker_data_print: Vec<TickerSchemaTypeData> = Vec::new();
        let packet_size = i16::from_be_bytes(int_bytes.try_into().unwrap());
        for _ in 0..packet_size {
            let (int_bytes, rest) = rest.split_at(std::mem::size_of::<i16>());
            let packet_length = i16::from_be_bytes(int_bytes.try_into().unwrap());
            println!("packet_length: {}", packet_length);
            let (packet_data, rest) = rest.split_at(packet_length as usize);
            let packet_schema_type = PacketSchemaType::from(packet_length).unwrap();
            let packet_schema_type_data =
                PacketSchemaTypeData::from2(packet_schema_type, packet_data);
            // ticker_data.push(BinaryKiteTickerMessageDecoder {
            //     packet_length,
            //     packet_schema_type_data,
            // });
            ticker_data_print.push(TickerSchemaTypeData::from_bin(packet_schema_type_data));
        }
        // BinaryKiteTickerMessage {
        //     packet_size,
        //     ticker_data,
        // }
        ticker_data_print
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::{
    //     fs::{File, OpenOptions},
    //     io::{Cursor, Seek, SeekFrom},
    // };
    #[test]
    fn test_kite_ticker_raw() -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(lines) = read_lines("./custom_mock_files/ws.packet") {
            for line in lines {
                if let Ok(line) = line {
                    // let data = &base64::decode(data).unwrap()[..];
                    let data = base64::decode(line.as_bytes()).unwrap();
                    // println!("{}", data.len());
                    // println!("{:?}", &data);
                    if data.len() > 2 {
                        // println!("{}", serde_json::to_string_pretty(&tick_data).unwrap());
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&BinaryKiteTickerMessage::from(data))
                                .unwrap()
                        );
                        // println!("{:#?}", &ticker_data);
                        // serde_json::to_writer_pretty(
                        //     &File::create("kite_ticker.json")?,
                        //     &ticker_data,
                        // )?;
                    }
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_kite_ticker_raw_single() -> Result<(), Box<dyn std::error::Error>> {
        // let data = b"AAQAuACfOQIAAABkAAAASwAAARwByROfABZ+NAAFYmsAAAG9AAAB9AAAAFoAAAPUYqGllwBC8qwATzXuADU/QGKhpZgAAGTIAAAAZAAkAAAAAJTtAAAAXwA8AAAAADZlAAAAWgAoAAAAABX5AAAAVQARAAAAADGcAAAAUAAZAAAAABydAAAAaQAoAAAAAFHWAAAAbgA5AAAAABftAAAAcwAWAAAAABnhAAAAeAAVAAAAAA+5AAAAfQAQAAAADAAAAQkAU8ibAFPCYQAMADcfAQAAH3cAAB9FAAgAA/gJADUe/A==";
        let data =
            b"AAUADAAJDwEAABhRAAAYVgAMAJ86AgAB1VYAAZZoAAwAnzkCAAAAZAAAA9QADAAD6QkAGPTPABj1KQAMAAABCQBTyJsAU8Jh";
        let data = base64::decode(data).unwrap();
        // println!("{}", data.len());
        // println!("{:?}", &data);
        if data.len() > 2 {
            // println!("{}", serde_json::to_string_pretty(&tick_data).unwrap());
            println!(
                "{}",
                serde_json::to_string_pretty(&BinaryKiteTickerMessage::from(data)).unwrap()
            );
            // println!("{:#?}", &BinaryKiteTickerMessage::from(data));
            // println!("{:#?}", &ticker_data);
            // serde_json::to_writer_pretty(&File::create("kite_ticker.json")?, &ticker_data)?;
        }
        Ok(())
    }
}
