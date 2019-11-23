// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2019 isis lovecruft, Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - isis agora lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

//! This module contains backend-specific constant values, such as the 64-bit limbs of curve constants.

use crate::u64::scalar::Scalar52;

/// `L` is the order of base point, i.e. 2^252 + 27742317777372353535851937790883648493
pub const L: Scalar52 = Scalar52([
    0x0002631a5cf5d3ed,
    0x000dea2f79cd6581,
    0x000000000014def9,
    0x0000000000000000,
    0x0000100000000000,
]);

/// `L` * `LFACTOR` = -1 (mod 2^52)
pub const LFACTOR: u64 = 0x51da312547e1b;

/// `R` = R % L where R = 2^260
pub const R: Scalar52 = Scalar52([
    0x000f48bd6721e6ed,
    0x0003bab5ac67e45a,
    0x000fffffeb35e51b,
    0x000fffffffffffff,
    0x00000fffffffffff,
]);

/// `RR` = (R^2) % L where R = 2^260
pub const RR: Scalar52 = Scalar52([
    0x0009d265e952d13b,
    0x000d63c715bea69f,
    0x0005be65cb687604,
    0x0003dceec73d217f,
    0x000009411b7c309a,
]);
