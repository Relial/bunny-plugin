#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum Num {
    Integer(i32),
    Float(f64),
}

impl Num {
    pub fn i32(num: i32) -> Self {
        Self::Integer(num)
    }

    pub fn f64(num: f64) -> Self {
        Self::Float(num)
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
