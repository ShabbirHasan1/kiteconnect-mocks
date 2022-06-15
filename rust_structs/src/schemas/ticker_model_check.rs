/*
[dependencies]
base64 = "*"
bytes = { version = "1.1.0", features = ["serde"] }
serde = { version = "1.0.137", features = [
    "derive",
    "alloc",
    "rc",
] }
serde_json = { version = "1.0.81", features = [
    "raw_value",
    "alloc",
    # "arbitrary_precision",
    "float_roundtrip",
    "indexmap",
    "preserve_order",
    "unbounded_depth",
] }
chrono = { version = "0.4.19", features = ["serde"] }
*/

use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

pub mod naive_date_time_timezone_from_timestamp {
    use chrono::{offset::LocalResult::Single, DateTime, FixedOffset, TimeZone};
    use serde::{de, ser, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let given_timestamp = Deserialize::deserialize(deserializer)?;
        if let Single(datetimeobject) = FixedOffset::east(19800).timestamp_opt(given_timestamp, 0) {
            Ok(datetimeobject)
        } else {
            Err(de::Error::custom("Invalid timestamp"))
        }
    }
    pub fn serialize<S>(dt: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // serializer
        //     .serialize_str(&dt.timestamp().to_string())
        //     .map_err(ser::Error::custom)
        serializer
            .serialize_str(&dt.to_rfc3339())
            .map_err(ser::Error::custom)
    }
}

fn convert_price(segment: i32, price: i32) -> f64 {
    let divisor_val: i32 = match segment {
        segment if segment == 3_i32 => 10_000_000_i32,
        segment if segment == 6_i32 => 10_000_i32,
        _ => 100_i32,
    };
    price as f64 / divisor_val as f64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepth {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedDepth {
    pub mode: String,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub last_traded_time: DateTime<FixedOffset>,
    pub extended_depth: MarketDepth,
}

impl Default for ExtendedDepth {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            is_tradable: Default::default(),
            token: Default::default(),
            last_price: Default::default(),
            last_traded_time: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            extended_depth: Default::default(),
        }
    }
}

impl From<&[u8]> for ExtendedDepth {
    fn from(mut buf: &[u8]) -> Self {
        let token = buf.get_i32();
        let segment = token & 0xFF;
        let is_index = segment == 9_i32;
        let last_price = convert_price(segment.into(), buf.get_i32());
        let last_traded_time = FixedOffset::east(19800).timestamp(buf.get_i32() as i64, 0);
        // let depth_buy = [0_u8, 20];
        // let depth_sell = [0_u8, 20];
        // let mut extended_depth = MarketDepth {
        //     buy: [MarketDepthData::default(); 20],
        //     sell: [MarketDepthData::default(); 20],
        // };
        // for i in 0..40 {
        //     if i < 20 {
        //        extended_depth.buy[i] = MarketDepthData {
        //                 quantity: buf.get_i32(),
        //                 price: convert_price(segment.into(), buf.get_i32()),
        //                 orders: buf.get_i32(),
        //             }
        //     } else {
        //         extended_depth.sell[i-20] = MarketDepthData {
        //                 quantity: buf.get_i32(),
        //                 price: convert_price(segment.into(), buf.get_i32()),
        //                 orders: buf.get_i32(),
        //             }
        //     }

        // }
        let extended_depth = MarketDepth {
            buy: [0_u8; 20]
                .into_iter()
                .map(|_| MarketDepthData {
                    quantity: buf.get_i32(),
                    price: convert_price(segment.into(), buf.get_i32()),
                    orders: buf.get_i32(),
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
            sell: [0_u8; 20]
                .into_iter()
                .map(|_| MarketDepthData {
                    quantity: buf.get_i32(),
                    price: convert_price(segment.into(), buf.get_i32()),
                    orders: buf.get_i32(),
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
        };
        Self {
            mode: "Full".into(),
            is_tradable: !is_index,
            token,
            last_price,
            last_traded_time,
            extended_depth,
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let a = base64::decode(b"AAEB7ADdngIAGAgXYqmAZQAAADIAGAeQAAAAAQAAADIAGAdyAAAAAQAAAGQAGAcsAAAAAgAAAGQAGAcJAAAAAQAAADIAGAcEAAAAAQAAADIAGAbmAAAAAQAAADIAGAbNAAAAAQAAADIAGAbIAAAAAQAAAJYAGAavAAAAAgAAAGQAGAabAAAAAgAAAiYAGAaWAAAAAwAAAGQAGAaMAAAAAQAAAGQAGAaHAAAAAgAAASwAGAaCAAAABQAAADIAGAZ9AAAAAQAAAGQAGAZ4AAAAAQAAAZAAGAZzAAAABQAAAfQAGAZuAAAABgAABUYAGAZpAAAADwAABdwAGAZkAAAACQAAADIAGAgcAAAAAQAAAGQAGAghAAAAAgAAAJYAGAgwAAAAAQAAADIAGAg1AAAAAQAAAPoAGAg6AAAAAgAAAGQAGAg/AAAAAgAAASwAGAhEAAAAAwAAAGQAGAhJAAAAAgAAASwAGAhOAAAABAAAASwAGAhTAAAABAAAAfQAGAhYAAAABgAAAPoAGAhdAAAAAwAAArwAGAhiAAAABQAAAcIAGAhnAAAAAwAAAcIAGAhsAAAABAAAASwAGAhxAAAAAwAAAfQAGAh2AAAABAAAAZAAGAh7AAAABAAAAcIAGAiAAAAABAAAAooAGAiFAAAACA==");
    // println!("{:?}", a);
    // let b = i32::from_be_bytes([0, 24, 15, 216]); //as f64 / 100_f64;
    // println!("{:?}", b);
    let mut buf = &a.unwrap()[..];
    print_type_of(&buf);
    println!("{:?}", buf.get_i16());
    println!("{:?}", buf.get_i16());
    let extended_depth = ExtendedDepth::from(buf);
    // println!("{:?}", &extended_depth);
    println!("{:?}", serde_json::to_string(&extended_depth).unwrap());
    // println!("{:?}", serde_json::to_string_pretty(&extended_depth).unwrap());
    // for _ in 0..41 {
    //     println!("{:?}", buf.get_i32());
    //     println!("{:?}", buf.get_i32());
    //     println!("{:?}", buf.get_i32());
    // }
    // println!("{:?}", buf.get_i32());
}

/*
[dependencies]
base64 = "*"
bytes = { version = "1.1.0", features = ["serde"] }
serde = { version = "1.0.137", features = [
    "derive",
    "alloc",
    "rc",
] }
serde_json = { version = "1.0.81", features = [
    "raw_value",
    "alloc",
    # "arbitrary_precision",
    "float_roundtrip",
    "indexmap",
    "preserve_order",
    "unbounded_depth",
] }
chrono = { version = "0.4.19", features = ["serde"] }
*/

use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

pub mod naive_date_time_timezone_from_timestamp {
    use chrono::{offset::LocalResult::Single, DateTime, FixedOffset, TimeZone};
    use serde::{de, ser, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let given_timestamp = Deserialize::deserialize(deserializer)?;
        if let Single(datetimeobject) = FixedOffset::east(19800).timestamp_opt(given_timestamp, 0) {
            Ok(datetimeobject)
        } else {
            Err(de::Error::custom("Invalid timestamp"))
        }
    }
    pub fn serialize<S>(dt: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // serializer
        //     .serialize_str(&dt.timestamp().to_string())
        //     .map_err(ser::Error::custom)
        serializer
            .serialize_str(&dt.to_rfc3339())
            .map_err(ser::Error::custom)
    }
}

fn convert_price(segment: i32, price: i32) -> f64 {
    let divisor_val: i32 = match segment {
        segment if segment == 3_i32 => 10_000_000_i32,
        segment if segment == 6_i32 => 10_000_i32,
        _ => 100_i32,
    };
    price as f64 / divisor_val as f64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepth {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedDepth {
    pub mode: String,
    pub is_tradable: bool,
    pub token: i32,
    pub last_price: f64,
    #[serde(with = "naive_date_time_timezone_from_timestamp")]
    pub last_traded_time: DateTime<FixedOffset>,
    pub extended_depth: MarketDepth,
}

impl Default for ExtendedDepth {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            is_tradable: Default::default(),
            token: Default::default(),
            last_price: Default::default(),
            last_traded_time: FixedOffset::east(19800).ymd(1947, 1, 1).and_hms(9, 15, 0),
            extended_depth: Default::default(),
        }
    }
}

impl From<Vec<u8>> for ExtendedDepth {
    fn from(buffer: Vec<u8>) -> Self {
        let mut buf = &buffer[..];
        println!("{:?}", buf.get_i16());
        println!("{:?}", buf.get_i16());
        let token = buf.get_i32();
        let segment = token & 0xFF;
        let is_index = segment == 9_i32;
        let last_price = convert_price(segment.into(), buf.get_i32());
        let last_traded_time = FixedOffset::east(19800).timestamp(buf.get_i32() as i64, 0);
        // let depth_buy = [0_u8, 20];
        // let depth_sell = [0_u8, 20];
        // let mut extended_depth = MarketDepth {
        //     buy: [MarketDepthData::default(); 20],
        //     sell: [MarketDepthData::default(); 20],
        // };
        // for i in 0..40 {
        //     if i < 20 {
        //        extended_depth.buy[i] = MarketDepthData {
        //                 quantity: buf.get_i32(),
        //                 price: convert_price(segment.into(), buf.get_i32()),
        //                 orders: buf.get_i32(),
        //             }
        //     } else {
        //         extended_depth.sell[i-20] = MarketDepthData {
        //                 quantity: buf.get_i32(),
        //                 price: convert_price(segment.into(), buf.get_i32()),
        //                 orders: buf.get_i32(),
        //             }
        //     }

        // }
        let extended_depth = MarketDepth {
            buy: [0_u8; 20]
                .into_iter()
                .map(|_| MarketDepthData {
                    quantity: buf.get_i32(),
                    price: convert_price(segment.into(), buf.get_i32()),
                    orders: buf.get_i32(),
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
            sell: [0_u8; 20]
                .into_iter()
                .map(|_| MarketDepthData {
                    quantity: buf.get_i32(),
                    price: convert_price(segment.into(), buf.get_i32()),
                    orders: buf.get_i32(),
                })
                .collect::<Vec<MarketDepthData>>()
                .try_into()
                .unwrap(),
        };
        Self {
            mode: "Full".into(),
            is_tradable: !is_index,
            token,
            last_price,
            last_traded_time,
            extended_depth,
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let a = base64::decode(b"AAEB7ADdngIAGAgXYqmAZQAAADIAGAeQAAAAAQAAADIAGAdyAAAAAQAAAGQAGAcsAAAAAgAAAGQAGAcJAAAAAQAAADIAGAcEAAAAAQAAADIAGAbmAAAAAQAAADIAGAbNAAAAAQAAADIAGAbIAAAAAQAAAJYAGAavAAAAAgAAAGQAGAabAAAAAgAAAiYAGAaWAAAAAwAAAGQAGAaMAAAAAQAAAGQAGAaHAAAAAgAAASwAGAaCAAAABQAAADIAGAZ9AAAAAQAAAGQAGAZ4AAAAAQAAAZAAGAZzAAAABQAAAfQAGAZuAAAABgAABUYAGAZpAAAADwAABdwAGAZkAAAACQAAADIAGAgcAAAAAQAAAGQAGAghAAAAAgAAAJYAGAgwAAAAAQAAADIAGAg1AAAAAQAAAPoAGAg6AAAAAgAAAGQAGAg/AAAAAgAAASwAGAhEAAAAAwAAAGQAGAhJAAAAAgAAASwAGAhOAAAABAAAASwAGAhTAAAABAAAAfQAGAhYAAAABgAAAPoAGAhdAAAAAwAAArwAGAhiAAAABQAAAcIAGAhnAAAAAwAAAcIAGAhsAAAABAAAASwAGAhxAAAAAwAAAfQAGAh2AAAABAAAAZAAGAh7AAAABAAAAcIAGAiAAAAABAAAAooAGAiFAAAACA==");
    print_type_of(&a);
    // // println!("{:?}", a);
    // // let b = i32::from_be_bytes([0, 24, 15, 216]); //as f64 / 100_f64;
    // // println!("{:?}", b);
    // let mut buf = &a.unwrap()[..];
    // print_type_of(&buf);
    // println!("{:?}", buf.get_i16());
    // println!("{:?}", buf.get_i16());
    let extended_depth = ExtendedDepth::from(a.unwrap());
    // println!("{:?}", &extended_depth);
    println!("{:?}", serde_json::to_string(&extended_depth).unwrap());
    // println!("{:?}", serde_json::to_string_pretty(&extended_depth).unwrap());
    // for _ in 0..41 {
    //     println!("{:?}", buf.get_i32());
    //     println!("{:?}", buf.get_i32());
    //     println!("{:?}", buf.get_i32());
    // }
    // println!("{:?}", buf.get_i32());
}
