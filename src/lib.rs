//! A basic ZipReader/Writer crate

#![warn(missing_docs)]

pub use crate::{
    compression::CompressionMethod, read::ZipArchive, types::DateTime, write::ZipWriter,
};

mod compression;
mod cp437;
mod crc32;
pub mod read;
pub mod result;
mod spec;
mod types;
pub mod write;
