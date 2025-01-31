//! This crate contains forrest data structures.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rust_2018_idioms, unused_lifetimes)]
#![warn(unused_qualifications, missing_docs)]
#![warn(clippy::all, clippy::panic)]
#![forbid(unsafe_code, clippy::expect_used)]

extern crate alloc;

#[cfg(test)]
extern crate std;

pub mod log;
pub mod map;

/// Types for converting to and from protobuf
pub mod protobuf {
    #![allow(clippy::all)]
    // Generated by [`prost-build`]
    include!(concat!(env!("OUT_DIR"), "/proofs.rs"));
    // Generated by [`pbjson-build`]
    include!(concat!(env!("OUT_DIR"), "/proofs.serde.rs"));
}
