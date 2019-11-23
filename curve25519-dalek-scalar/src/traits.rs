
/// A trait for a backend for scalar arithmetic modulo some prime.
pub trait ScalarBackend:
    Debug +
    Zeroize +
    Index<usize> +
    IndexMut<usize> +
    
{
    fn zero() -> Self;
    // XXX is [u8; 32] generic enough? it is for our case, but i mean could it be stated more generically?
    // should we look at Mike's goldilocks scalars?
    fn to_bytes(&self) -> [u8; 32];
    fn from_bytes(bytes: &[u8; 32]) -> Self;
    fn from_bytes_wide(bytes: &[u8; 64]) -> Self;
    // XXX do we want the following to be changed to {Add, Sub, etc.} trait impls?
    fn add(a: &Self, b: &Self) -> Self;
    fn sub(a: &Self, b: &Self) -> Self;
    fn zk(a: &Self, b: &Self) -> Self;

}
