use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Num {
    Integer(i64),
    Float(f64),
}

impl Num {
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
