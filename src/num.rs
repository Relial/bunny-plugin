#[repr(C)]
pub enum Num<'a> {
    F32(&'a mut f32),
    F64(&'a mut f64),
    U8(&'a mut u8),
    U16(&'a mut u16),
    U32(&'a mut u32),
    U64(&'a mut u64),
    I8(&'a mut i8),
    I16(&'a mut i16),
    I32(&'a mut i32),
    I64(&'a mut i64),
}

impl Num<'_> {
    pub fn integer(&self) -> bool {
        !matches!(self, Self::F32(_) | Self::F64(_))
    }

    pub fn set(&mut self, value: f64) {
        match self {
            Num::F32(f) => **f = value as f32,
            Num::F64(f) => **f = value,
            Num::U8(u) => **u = value as u8,
            Num::U16(u) => **u = value as u16,
            Num::U32(u) => **u = value as u32,
            Num::U64(u) => **u = value as u64,
            Num::I8(i) => **i = value as i8,
            Num::I16(i) => **i = value as i16,
            Num::I32(i) => **i = value as i32,
            Num::I64(i) => **i = value as i64,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Num::F32(f) => **f as f64,
            Num::F64(f) => **f,
            Num::U8(u) => **u as f64,
            Num::U16(u) => **u as f64,
            Num::U32(u) => **u as f64,
            Num::U64(u) => **u as f64,
            Num::I8(i) => **i as f64,
            Num::I16(i) => **i as f64,
            Num::I32(i) => **i as f64,
            Num::I64(i) => **i as f64,
        }
    }

    pub fn min(&self) -> f64 {
        match self {
            Num::F32(_) | Num::F64(_) => f64::NEG_INFINITY,
            Num::U8(_) | Num::U16(_) | Num::U32(_) | Num::U64(_) => 0.0,
            Num::I8(_) => i8::MIN as f64,
            Num::I16(_) => i16::MIN as f64,
            Num::I32(_) => i32::MIN as f64,
            Num::I64(_) => i64::MIN as f64,
        }
    }

    pub fn max(&self) -> f64 {
        match self {
            Num::F32(_) | Num::F64(_) => f64::INFINITY,
            Num::U8(_) => u8::MAX as f64,
            Num::U16(_) => u16::MAX as f64,
            Num::U32(_) => u32::MAX as f64,
            Num::U64(_) => u64::MAX as f64,
            Num::I8(_) => i8::MAX as f64,
            Num::I16(_) => i16::MAX as f64,
            Num::I32(_) => i32::MAX as f64,
            Num::I64(_) => i64::MAX as f64,
        }
    }
}

impl<'a> From<&'a mut f32> for Num<'a> {
    fn from(value: &'a mut f32) -> Self {
        Self::F32(value)
    }
}

impl<'a> From<&'a mut f64> for Num<'a> {
    fn from(value: &'a mut f64) -> Self {
        Self::F64(value)
    }
}

impl<'a> From<&'a mut u8> for Num<'a> {
    fn from(value: &'a mut u8) -> Self {
        Self::U8(value)
    }
}

impl<'a> From<&'a mut u16> for Num<'a> {
    fn from(value: &'a mut u16) -> Self {
        Self::U16(value)
    }
}

impl<'a> From<&'a mut u32> for Num<'a> {
    fn from(value: &'a mut u32) -> Self {
        Self::U32(value)
    }
}

impl<'a> From<&'a mut u64> for Num<'a> {
    fn from(value: &'a mut u64) -> Self {
        Self::U64(value)
    }
}

impl<'a> From<&'a mut i8> for Num<'a> {
    fn from(value: &'a mut i8) -> Self {
        Self::I8(value)
    }
}

impl<'a> From<&'a mut i16> for Num<'a> {
    fn from(value: &'a mut i16) -> Self {
        Self::I16(value)
    }
}

impl<'a> From<&'a mut i32> for Num<'a> {
    fn from(value: &'a mut i32) -> Self {
        Self::I32(value)
    }
}

impl<'a> From<&'a mut i64> for Num<'a> {
    fn from(value: &'a mut i64) -> Self {
        Self::I64(value)
    }
}

// #[repr(C)]
// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
// pub enum NumBad {
//     Integer(i64),
//     Float(f64),
// }

// impl NumBad {
//     pub const INT_MIN: f64 = i64::MIN as f64;
//     pub const INT_MAX: f64 = i64::MAX as f64;

//     pub fn new_integer(integer: impl Into<i64>) -> Self {
//         Self::Integer(integer.into())
//     }

//     pub fn new_float(float: impl Into<f64>) -> Self {
//         Self::Float(float.into())
//     }

//     pub const fn int(self) -> bool {
//         matches!(self, Self::Integer(_))
//     }

//     pub const fn float(self) -> bool {
//         matches!(self, Self::Float(_))
//     }

//     pub fn set(&mut self, value: f64) {
//         match self {
//             Num::Integer(int) => *int = value as i64,
//             Num::Float(float) => *float = value,
//         }
//     }

//     pub fn to_f64(self) -> f64 {
//         match self {
//             Num::Integer(int) => int as f64,
//             Num::Float(float) => float,
//         }
//     }
// }

// impl From<Num> for f64 {
//     fn from(value: Num) -> Self {
//         match value {
//             Num::Integer(i) => i as f64,
//             Num::Float(f) => f,
//         }
//     }
// }

// impl From<Num> for i64 {
//     fn from(value: Num) -> Self {
//         match value {
//             Num::Integer(i) => i,
//             Num::Float(f) => f as i64,
//         }
//     }
// }

// impl From<i64> for Num {
//     fn from(value: i64) -> Self {
//         Self::Integer(value)
//     }
// }

// impl From<f64> for Num {
//     fn from(value: f64) -> Self {
//         Self::Float(value)
//     }
// }

// macro_rules! partialord_impl {
//     ($t:ty) => {
//         impl PartialEq<$t> for Num {
//             fn eq(&self, other: &$t) -> bool {
//                 match self {
//                     Num::Integer(i) => *i == *other as i64,
//                     Num::Float(f) => *f == *other as f64,
//                 }
//             }
//         }

//         impl PartialEq<Num> for $t {
//             fn eq(&self, other: &Num) -> bool {
//                 match other {
//                     Num::Integer(i) => *self as i64 == *i,
//                     Num::Float(f) => *self as f64 == *f,
//                 }
//             }
//         }

//         impl PartialOrd<$t> for Num {
//             fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
//                 match self {
//                     Num::Integer(i) => i.partial_cmp(&(*other as i64)),
//                     Num::Float(f) => f.partial_cmp(&(*other as f64)),
//                 }
//             }
//         }

//         impl PartialOrd<Num> for $t {
//             fn partial_cmp(&self, other: &Num) -> Option<std::cmp::Ordering> {
//                 match other {
//                     Num::Integer(i) => (*self as i64).partial_cmp(i),
//                     Num::Float(f) => (*self as f64).partial_cmp(f),
//                 }
//             }
//         }
//     };
// }

// partialord_impl!(i64);
// partialord_impl!(i32);
// partialord_impl!(i16);
// partialord_impl!(i8);

// partialord_impl!(u32);
// partialord_impl!(u16);
// partialord_impl!(u8);

// partialord_impl!(f64);
// partialord_impl!(f32);
