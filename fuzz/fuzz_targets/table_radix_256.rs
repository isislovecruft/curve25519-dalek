#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate curve25519_dalek;

use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use curve25519_dalek::constants::ED25519_BASEPOINT_TABLE;
use curve25519_dalek::edwards::EdwardsBasepointTableRadix256;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::BasepointTable;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 32 {
        let mut bytes = [0u8; 32];

        bytes.copy_from_slice(&data[..32]);

        let x = Scalar::from_bits(bytes);
        let P = ED25519_BASEPOINT_POINT;

        let table = EdwardsBasepointTableRadix256::create(&P);

        let Q = &x * &ED25519_BASEPOINT_TABLE;
        let Q_prime = &x * &table;

        assert_eq!(Q.compress(), Q_prime.compress());
    }
});
