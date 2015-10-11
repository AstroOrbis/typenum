use std::cmp::{Ordering};

pub mod bit;

pub mod uint;
pub mod consts;

pub mod int;

pub mod __private;

pub trait Same<Rhs = Self> {
    /// `Output` should always be `Self`
    type Output;
}

pub trait Sub<Rhs = Self> {
    type Output;
}
pub trait Mul<Rhs = Self> {
    type Output;
}
pub trait Div<Rhs = Self> {
    type Output;
}


pub trait Ord {
    fn to_ordering() -> Ordering;
}

pub struct Greater;
pub struct Less;
pub struct Equal;

impl Ord for Greater {
    fn to_ordering() -> Ordering { Ordering::Greater }
}
impl Ord for Less {
    fn to_ordering() -> Ordering { Ordering::Less }
}
impl Ord for Equal {
    fn to_ordering() -> Ordering { Ordering::Equal }
}

/// Compares `Self` and `Rhs`. Should only ever return one of `Greater`, `Less`, or `Equal`.
pub trait Cmp<Rhs = Self> {
    type Output;
}

/// Gives the size of a type number in bits as a `UInt`
pub trait SizeOf {
    type Output;
}
