#![deny(missing_docs)]
#![allow(missing_copy_implementations)]

//! dox (placeholder)

extern crate byteorder;
extern crate rand;

#[macro_use]
extern crate log;

#[cfg(test)]
#[cfg(feature = "unstable")]
extern crate test;

use std::io::{self, Read};

/// Public exports
#[cfg(feature = "checksum")]
pub use self::checksum::adler::State32 as Adler32;

#[cfg(feature = "checksum")]
/// Checksum algorithms. Requires `checksum` feature, enabled by default
// http://en.wikipedia.org/wiki/Checksum
pub mod checksum {
    pub mod adler;
}

#[cfg(feature = "bwt")]
pub mod bwt;

#[cfg(feature = "flate")]
pub mod flate;

#[cfg(feature = "lz4")]
pub mod lz4;

#[cfg(feature = "zlib")]
pub mod zlib;

/// Entropy coder family. Requires `entropy` feature, enabled by default
// http://en.wikipedia.org/wiki/Entropy_encoding
#[cfg(feature = "entropy")]
pub mod entropy {
    pub mod ari;
}

#[cfg(feature = "rle")]
pub mod rle;

/// Adds a convenience method for types with the read trait, very similar
/// to push_at_least in the late Reader trait
pub trait ReadExact: Read + Sized {
    /// Appends exact number of bytes to a buffer
    fn push_exactly(&mut self, bytes: u64, buf: &mut Vec<u8>) -> io::Result<()> {
        let n = r#try!(self.by_ref().take(bytes).read_to_end(buf)) as u64;

        if n < bytes {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "unexpected end of file",
            ));
        }

        Ok(())
    }
}

impl<T> ReadExact for T where T: Read + Sized {}
