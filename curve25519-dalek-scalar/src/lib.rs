// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2019 isis lovecruft, Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - isis agora lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

#[cfg(feature = "u32_backend")]
mod u32;
#[cfg(feature = "u32_backend")]
pub use crate::u32::constants;

#[cfg(feature = "u64_backend")]
mod u64;
#[cfg(feature = "u64_backend")]
pub use crate::u64::constants;

pub mod scalar;

pub use scalar::UnpackedScalar;
