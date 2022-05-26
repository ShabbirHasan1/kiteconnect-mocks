// Package checksums (i.e. SHA-256 digests)

use serde::{de, ser, Deserialize, Serialize};
use std::{fmt, io, str::FromStr};

// Result type with the `cargo-lock` crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

// Error type.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    // An error occurred performing an I/O operation (e.g. network, file)
    Io(io::ErrorKind),

    // Couldn't parse response data
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(kind) => write!(f, "I/O operation failed: {:?}", kind),
            Error::Parse(s) => write!(f, "parse error: {}", s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err.kind())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parse(err.to_string())
    }
}

impl std::error::Error for Error {}

// Cryptographic checksum (SHA-256) for a package
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Checksum {
    // SHA-256 digest of a package
    Sha256([u8; 32]),
}

impl Default for Checksum {
    fn default() -> Self {
        Checksum::Sha256([0; 32])
    }
}

impl Checksum {
    // Is this checksum SHA-256?
    pub fn is_sha256(&self) -> bool {
        self.as_sha256().is_some()
    }

    // If this is a SHA-256 checksum, get the raw bytes
    pub fn as_sha256(&self) -> Option<[u8; 32]> {
        match self {
            Checksum::Sha256(digest) => Some(*digest),
        }
    }
}

impl From<[u8; 32]> for Checksum {
    fn from(bytes: [u8; 32]) -> Checksum {
        Checksum::Sha256(bytes)
    }
}

impl FromStr for Checksum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 64 {
            return Err(Error::Parse(format!(
                "invalid checksum: expected 64 hex chars, got {}",
                s.len()
            )));
        }

        let mut digest = [0u8; 32];

        for (i, byte) in digest.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&s[(i * 2)..=(i * 2) + 1], 16)?;
        }

        Ok(Checksum::Sha256(digest))
    }
}

impl fmt::Debug for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Checksum::Sha256(_) => write!(f, "Sha256({:x})", self),
        }
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl fmt::LowerHex for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Checksum::Sha256(digest) => {
                for b in digest {
                    write!(f, "{:02x}", b)?;
                }
            }
        }

        Ok(())
    }
}

impl fmt::UpperHex for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Checksum::Sha256(digest) => {
                for b in digest {
                    write!(f, "{:02X}", b)?;
                }
            }
        }

        Ok(())
    }
}

impl<'de> Deserialize<'de> for Checksum {
    fn deserialize<D: de::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let hex = String::deserialize(deserializer)?;
        hex.parse().map_err(de::Error::custom)
    }
}

impl Serialize for Checksum {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_round_trip() {
        let checksum_str = "af6f3550d8dff9ef7dc34d384ac6f107e5d31c8f57d9f28e0081503f547ac8f5";
        let checksum = checksum_str.parse::<Checksum>().unwrap();
        // println!("{checksum_str}\n{checksum}");
        assert_eq!(checksum_str, checksum.to_string());
    }

    #[test]
    fn invalid_checksum() {
        // Missing one hex letter
        let invalid_str = "af6f3550d8dff9ef7dc34d384ac6f107e5d31c8f57d9f28e0081503f547ac8f";
        let error = invalid_str.parse::<Checksum>().err().unwrap();
        // println!("{invalid_str}\n{error}");
        assert!(matches!(error, Error::Parse(_)));
    }
}
