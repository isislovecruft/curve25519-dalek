// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2019 isis lovecruft, Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - isis agora lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

//! This module contains various constants (such as curve parameters
//! and useful field elements like `sqrt(-1)`), as well as
//! lookup tables of pre-computed points.

use crate::u32::scalar::Scalar29;

/// `L` is the order of base point, i.e. 2^252 +
/// 27742317777372353535851937790883648493
pub const L: Scalar29 = Scalar29([
    0x1cf5d3ed, 0x009318d2, 0x1de73596, 0x1df3bd45, 0x0000014d, 0x00000000, 0x00000000, 0x00000000,
    0x00100000,
]);

/// `L` * `LFACTOR` = -1 (mod 2^29)
pub const LFACTOR: u32 = 0x12547e1b;

/// `R` = R % L where R = 2^261
pub const R: Scalar29 = Scalar29([
    0x114df9ed, 0x1a617303, 0x0f7c098c, 0x16793167, 0x1ffd656e, 0x1fffffff, 0x1fffffff, 0x1fffffff,
    0x000fffff,
]);

/// `RR` = (R^2) % L where R = 2^261
pub const RR: Scalar29 = Scalar29([
    0x0b5f9d12, 0x1e141b17, 0x158d7f3d, 0x143f3757, 0x1972d781, 0x042feb7c, 0x1ceec73d, 0x1e184d1e,
    0x0005046d,
]);
