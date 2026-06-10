#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum Num {
    Integer(i32),
    Float(f64),
}

impl Num {
    pub fn integer(integer: impl Into<i32>) -> Self {
        Self::Integer(integer.into())
    }

    pub fn float(float: impl Into<f64>) -> Self {
        Self::Float(float.into())
    }
}

impl From<Num> for f64 {
    fn from(value: Num) -> Self {
        match value {
            Num::Integer(i) => i.into(),
            Num::Float(f) => f,
        }
    }
}

impl From<Num> for i32 {
    fn from(value: Num) -> Self {
        match value {
            Num::Integer(i) => i,
            Num::Float(f) => f as i32,
        }
    }
}

impl From<i32> for Num {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Num {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
