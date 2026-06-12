use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Num {
    Integer(i64),
    Float(f64),
}

impl Num {
    pub const INT_MIN: f64 = i64::MIN as f64;
    pub const INT_MAX: f64 = i64::MAX as f64;

    pub fn new_integer(integer: impl Into<i64>) -> Self {
        Self::Integer(integer.into())
    }

    pub fn new_float(float: impl Into<f64>) -> Self {
        Self::Float(float.into())
    }

    pub const fn int(self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub const fn float(self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn set(&mut self, value: f64) {
        match self {
            Num::Integer(int) => *int = value as i64,
            Num::Float(float) => *float = value,
        }
    }

    pub fn to_f64(self) -> f64 {
        match self {
            Num::Integer(int) => int as f64,
            Num::Float(float) => float,
        }
    }
}

impl From<Num> for f64 {
    fn from(value: Num) -> Self {
        match value {
            Num::Integer(i) => i as f64,
            Num::Float(f) => f,
        }
    }
}

impl From<Num> for i64 {
    fn from(value: Num) -> Self {
        match value {
            Num::Integer(i) => i,
            Num::Float(f) => f as i64,
        }
    }
}

impl From<i64> for Num {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Num {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

macro_rules! partialord_impl {
    ($t:ty) => {
        impl PartialEq<$t> for Num {
            fn eq(&self, other: &$t) -> bool {
                match self {
                    Num::Integer(i) => *i == *other as i64,
                    Num::Float(f) => *f == *other as f64,
                }
            }
        }

        impl PartialEq<Num> for $t {
            fn eq(&self, other: &Num) -> bool {
                match other {
                    Num::Integer(i) => *self as i64 == *i,
                    Num::Float(f) => *self as f64 == *f,
                }
            }
        }

        impl PartialOrd<$t> for Num {
            fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                match self {
                    Num::Integer(i) => i.partial_cmp(&(*other as i64)),
                    Num::Float(f) => f.partial_cmp(&(*other as f64)),
                }
            }
        }

        impl PartialOrd<Num> for $t {
            fn partial_cmp(&self, other: &Num) -> Option<std::cmp::Ordering> {
                match other {
                    Num::Integer(i) => (*self as i64).partial_cmp(i),
                    Num::Float(f) => (*self as f64).partial_cmp(f),
                }
            }
        }
    };
}

partialord_impl!(i64);
partialord_impl!(i32);
partialord_impl!(i16);
partialord_impl!(i8);

partialord_impl!(u32);
partialord_impl!(u16);
partialord_impl!(u8);

partialord_impl!(f64);
partialord_impl!(f32);
