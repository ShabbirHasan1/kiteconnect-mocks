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

use rayon::prelude::*;
use serde::Deserialize;
use std::io::Read;

/// This describes the following JSON object: {"f1": 34}
#[derive(Deserialize, Debug)]
struct LogLine {
    f1: u32,
}

fn last_newline(s: &[u8]) -> usize {
    let mut i = s.len() - 1;
    while i > 0 {
        if s[i] == b'\n' {
            return i + 1;
        }
        i -= 1;
    }
    s.len()
}

fn process_line(v: LogLine) {
    println!("X={}", v.f1);
}

const CHUNK_SIZE: usize = 100_000_000;

fn main() {
    // By using rayon::scope, it will wait for everything we spawn inside.
    rayon::scope(|scope| {
        let mut fd = std::fs::File::open("logs").unwrap();

        let mut s = Vec::with_capacity(CHUNK_SIZE);
        loop {
            // Read at most 100 MB into s.
            fd.by_ref()
                .take((CHUNK_SIZE - s.len()) as u64)
                .read_to_end(&mut s)
                .unwrap();

            if s.len() == 0 {
                // The file has ended.
                break;
            }

            // Copy any incomplete lines to the next s.
            let last_newline = last_newline(&s);
            let mut next_s = Vec::with_capacity(CHUNK_SIZE);
            next_s.extend_from_slice(&s[last_newline..]);
            s.truncate(last_newline);

            // Move our string into a rayon thread.
            let data = s;
            scope.spawn(move |_| {
                data[..last_newline]
                    .split(|c| *c == b'\n')
                    .par_bridge()
                    .filter_map(|line| serde_json::from_slice(line).ok())
                    .for_each(process_line);
            });

            s = next_s;
        }
    });
}

// 1.

use rayon::prelude::*;
use serde::Deserialize;
use std::io::BufRead;

/// This describes the following JSON object: {"f1": 34}
#[derive(Deserialize, Debug)]
struct LogLine {
    f1: u32,
}

fn main() {
    let fd = std::fs::File::open("logs").unwrap();
    let x = std::io::BufReader::new(fd);

    x.lines() // split to lines serially
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge() // parallelize
        .filter_map(|line: String| serde_json::from_str(&line).ok()) // filter out bad lines
        .for_each(|v: LogLine| {
            // do some processing (in parallel)
            println!("X={}", v.f1);
        });
}

// alice
// Regular
// May '20
// You can use a single string buffer like this:

use rayon::prelude::*;
use serde::Deserialize;
use std::io::Read;

/// This describes the following JSON object: {"f1": 34}
#[derive(Deserialize, Debug)]
struct LogLine {
    f1: u32,
}

fn main() {
    let mut fd = std::fs::File::open("logs").unwrap();
    let len = fd.metadata().unwrap().len() as usize;
    let mut file = String::with_capacity(len);
    fd.read_to_string(&mut file).unwrap(); // This reads the entire file into memory.
    drop(fd);

    file.lines() // split to lines serially
        .par_bridge() // parallelize
        .filter_map(|line| serde_json::from_str(line).ok())
        .for_each(|v: LogLine| {
            // do some processing (in parallel)
            println!("X={}", v.f1);
        });
}

// This uses str::lines instead of BufRead::lines, which returns references into the same string instead of a new allocation per line.

// Reuse the same String buffer
fn read_line(file_name: &str, func: fn(&str)) -> Result<(), std::io::Error> {
    // open target file
    let file = File::open(&file_name)?;

    // uses a reader buffer
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                // EOF: save last file address to restart from this address for next run
                if bytes_read == 0 {
                    break;
                }

                func(&line);

                // do not accumulate data
                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    Ok(())
}



// -------------------------------------------------------------------------------------------
use std::mem;

enum MaybeOwned<'a, T> {
    Ref(&'a T),
    Boxed(Box<T>)
}

impl<'a,T> std::ops::Deref for MaybeOwned<'a, T> {
    type Target=T;
    fn deref(&self)->&T {
        match self {
            Self::Ref(r) => r,
            Self::Boxed(ref x) => &*x
        }
    }
}

#[cfg(target_endian = "little")]
unsafe trait FromLEByteSlice: Sized {
    fn cast_aligned<'a>(bytes: &'a [u8]) -> Option<&'a Self> {
        assert_eq!(bytes.len(), mem::size_of::<Self>());
        let ptr: *const u8 = bytes.as_ptr();
        match ptr.align_offset(mem::align_of::<Self>()) {
            0 => unsafe { ptr.cast::<Self>().as_ref() },
            _ => None
        }
    }

    fn read_unaligned(bytes: & [u8]) -> Self {
        assert_eq!(bytes.len(), mem::size_of::<Self>());
        unsafe { bytes.as_ptr().cast::<Self>().read_unaligned() }
    }

    fn cast<'a>(bytes: &'a [u8]) -> MaybeOwned<'a, Self> {
        match Self::cast_aligned(bytes) {
            Some(x) => MaybeOwned::Ref(x),
            None => MaybeOwned::Boxed(Box::new(Self::read_unaligned(bytes)))
        }
    }
}
// -------------------------------------------------------------------------------------------
unsafe fn cast_ref<'a, T>(bytes: &'a [u8]) -> &'a T {
    // assert correct endianness somehow
    assert_eq!(bytes.len(), mem::size_of::<T>());
    let ptr: *const u8 = bytes.as_ptr();
    assert_eq!(ptr.align_offset(mem::align_of::<T>()), 0);

    ptr.cast::<T>().as_ref().unwrap()
}
// -------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct IdentifyControllerResponse {
    // PCI Vendor ID
    pub vid: u16,
    // PCI Subsystem Vendor ID
    pub svid: u16,
    // Serial Number
    pub sn: [u8; 20],
    // Model Number
    pub mn: [u8; 40],
    // Firmware Revision
    pub fr: [u8; 8],
    // Recommended Arbitration Burst
    pub rab: u8,
    // IEEE OUI Identifier
    pub ieee: [u8; 3],
    // Controller Multi-Path I/O and Namespace Sharing Capabilities
    pub cmic: u8,
    // Maximum Data Transfer Size
    pub mdts: u8,
    // Controller ID
    pub cntlid: u16,
    // Version
    pub ver: u32,
    // RTD3 Resume Latency
    pub rtd3r: u32,
    // RTD3 Entry Latency
    pub rtd3e: u32,
    // Optional Asynchronous Events Supported
    pub oaes: u32,
    // Controller Attributes
    pub ctratt: u32,
    // Read Recovery Levels Supported
    pub rrls: u16,
    _rsvd1: [u8; 9],
    // Controller Type
    pub cntrltype: u8,
    // FRU Globally Unique Identifier
    pub fguid: u128,
    // Command Retry Delay Times
    pub crdt: [u16; 3],
    _rsvd2: [u8; 119],
    // NVM Subsystem Report
    pub nvmsr: u8,
    // VPD Write Cycle Information
    pub vwci: u8,
    // Management Endpoint Capabilities
    pub mec: u8,
    // Optional Admin Command Support
    pub oacs: u16,
    // Abort Command Limit
    pub acl: u8,
    // Asynchronous Event Request Limit
    pub aerl: u8,
    // Firmware Updates
    pub frmw: u8,
    // Log Page Attributes
    pub lpa: u8,
    // Error Log Page Entries
    pub elpe: u8,
    // Number of Power States Support
    pub npss: u8,
    // Admin Vendor Specific Command Configuration
    pub avscc: u8,
    // Autonomous Power State Transition Attributes
    pub apsta: u8,
    // Warning Composite Temperature Threshold
    pub wctemp: u16,
    // Critical Composite Temperature Threshold
    pub cctemp: u16,
    // Maximum Time for Firmware Activation
    pub mtfa: u16,
    // Host Memory Buffer Preferred Size
    pub hmpre: u32,
    // Host Memory Buffer Minimum Size
    pub hmmin: u32,
    // Total NVM Capacity
    pub tnvmcap: u128,
    // Unallocated NVM Capacity
    pub unvmcap: u128,
    // Replay Protected Memory Block Support
    pub rpmbs: u32,
    // Extended Device Self-test Time
    pub edstt: u16,
    // Device Self-test Options
    pub dsto: u8,
    // Firmware Update Granularity
    pub fwug: u8,
    // Keep Alive Support
    pub kas: u16,
    // Host Controlled Thermal Management Attributes
    pub hctma: u16,
    // Minimum Thermal Management Temperature
    pub mntmt: u16,
    // Maximum Thermal Management Temperature
    pub mxtmt: u16,
    // Sanitize Capabilities
    pub sanicap: u32,
    // Host Memory Buffer Minimum Descriptor Entry Size
    pub hmminds: u32,
    // Host Memory Maximum Descriptors Entries
    pub hmmaxd: u16,
    // NVM Set Identifier Maximum
    pub nsetidmax: u16,
    // Endurance Group Identifier Maximum
    pub endgidmax: u16,
    // ANA Transition Time
    pub anatt: u8,
    // Asymmetric Namespace Access Capabilities
    pub anacap: u8,
    // ANA Group Identifier Maximum
    pub anagrpmax: u32,
    // Number of ANA Group Identifiers
    pub nanagrpid: u32,
    // Persistent Event Log Size
    pub pels: u32,
    _rsvd3: [u8; 156],
    // Submission Queue Entry Size
    pub sqes: u8,
    // Completion Queue Entry Size
    pub cqes: u8,
    // Maximum Outstanding Commands
    pub maxcmd: u16,
    // Number of Namespaces
    pub nn: u32,
    // Optional NVM Command Support
    pub oncs: u16,
    // Fused Operation Support
    pub fuses: u16,
    // Format NVM Attributes
    pub fna: u8,
    // Volatile Write Cache
    pub vwc: u8,
    // Atomic Write Unit Normal
    pub awun: u16,
    // Atomic Write Unit Power Fail
    pub awupf: u16,
    // NVM Vendor Specific Command Configuration
    pub nvscc: u8,
    // Namespace Write Protection Capabilities
    pub nwpc: u8,
    // Atomic Compare & Write Unit
    pub acwu: u16,
    _rsvd4: u16,
    // SGL Support
    pub sgls: u32,
    // Maximum Number of Allowed Namespaces
    pub mnan: u32,
    _rsvd5: [u8; 224],
    // NVM Subsystem NVMe Qualified Name
    pub subnqn: [u8; 256],
    _rsvd6: [u8; 768],
    // I/O Queue Command Capsule Supported Size
    pub ioccsz: u32,
    // I/O Queue Response Capsule Supported Size
    pub iorcsz: u32,
    // In Capsule Data Offset
    pub icdoff: u16,
    // Fabrics Controller Attributes
    pub fcatt: u8,
    // Maximum SGL Data Block Descriptors
    pub msdbd: u8,
    // Optional Fabric Commands Support
    pub ofcs: u16,
    _rsvd7: [u8; 242],
    // Power State Descriptors
    pub psd: [[u128; 2]; 31],
    // Vendor Specific
    pub vs: [u8; 1024],
}
assert_eq_size!(IdentifyControllerResponse, [u8; 4096]);
// -------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------
