use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

mod dynamic;
mod r#static;

pub use digest::{Digest, Output};
pub use sha2::Sha256;

pub use dynamic::{DynHash, DynHashParseError};
pub use r#static::Hash;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum HashAlgorithm {
    Sha256,
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithm::Sha256 => write!(f, "sha256"),
        }
    }
}

impl FromStr for HashAlgorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sha256" => Ok(HashAlgorithm::Sha256),
            _ => Err(Error::msg(format!("Illegal hash algorithm '{}'", s))),
        }
    }
}

pub trait SupportedDigest: Digest + private::Sealed {
    const ALGORITHM: HashAlgorithm;
}

impl SupportedDigest for Sha256 {
    const ALGORITHM: HashAlgorithm = HashAlgorithm::Sha256;
}

mod private {
    use sha2::Sha256;

    pub trait Sealed {}
    impl Sealed for Sha256 {}
}

impl<D: SupportedDigest> From<Hash<D>> for DynHash {
    fn from(value: Hash<D>) -> Self {
        DynHash {
            algo: D::ALGORITHM,
            bytes: value.digest.to_vec(),
        }
    }
}

impl<D: SupportedDigest> TryFrom<DynHash> for Hash<D> {
    type Error = Error;

    fn try_from(value: DynHash) -> Result<Self, Self::Error> {
        if value.algorithm() == D::ALGORITHM {
            let hash = Hash::try_from(value.bytes)?;
            Ok(hash)
        } else {
            Err(Error::msg("Mismatched algorithms"))
        }
    }
}
