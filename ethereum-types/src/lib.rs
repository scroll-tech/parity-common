// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]

mod hash;
mod uint;

#[cfg(feature = "ethbloom")]
pub use ethbloom::{Bloom, BloomRef, Input as BloomInput};
pub use hash::{BigEndianHash, H128, H160, H256, H264, H32, H512, H520, H64};
pub use uint::{FromDecStrErr, FromStrRadixErr, FromStrRadixErrKind, U128, U256, U512, U64};

#[cfg(feature = "rkyv")]
pub use hash::{ArchivedH128, ArchivedH160, ArchivedH256, ArchivedH32, ArchivedH512, ArchivedH520, ArchivedH64};
#[cfg(feature = "rkyv")]
pub use uint::{ArchivedU128, ArchivedU256, ArchivedU512, ArchivedU64};

pub type Address = H160;
pub type Secret = H256;
pub type Public = H512;
pub type Signature = H520;

#[cfg(feature = "rkyv")]
pub type ArchivedAddress = ArchivedH160;
#[cfg(feature = "rkyv")]
pub type ArchivedSecret = ArchivedH256;
#[cfg(feature = "rkyv")]
pub type ArchivedPublic = ArchivedH512;
#[cfg(feature = "rkyv")]
pub type ArchivedSignature = ArchivedH520;

/// Conditional compilation depending on whether ethereum-types is built with ethbloom support.
#[cfg(feature = "ethbloom")]
#[macro_export]
macro_rules! if_ethbloom {
    ($($tt:tt)*) => {
        $($tt)*
    };
}

#[cfg(not(feature = "ethbloom"))]
#[macro_export]
#[doc(hidden)]
macro_rules! if_ethbloom {
	($($tt:tt)*) => {};
}
