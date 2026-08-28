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
    #[inline]
    pub fn integer(&self) -> bool {
        !matches!(self, Self::F32(_) | Self::F64(_))
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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
    #[inline]
    fn from(value: &'a mut f32) -> Self {
        Self::F32(value)
    }
}

impl<'a> From<&'a mut f64> for Num<'a> {
    #[inline]
    fn from(value: &'a mut f64) -> Self {
        Self::F64(value)
    }
}

impl<'a> From<&'a mut u8> for Num<'a> {
    #[inline]
    fn from(value: &'a mut u8) -> Self {
        Self::U8(value)
    }
}

impl<'a> From<&'a mut u16> for Num<'a> {
    #[inline]
    fn from(value: &'a mut u16) -> Self {
        Self::U16(value)
    }
}

impl<'a> From<&'a mut u32> for Num<'a> {
    #[inline]
    fn from(value: &'a mut u32) -> Self {
        Self::U32(value)
    }
}

impl<'a> From<&'a mut u64> for Num<'a> {
    #[inline]
    fn from(value: &'a mut u64) -> Self {
        Self::U64(value)
    }
}

impl<'a> From<&'a mut i8> for Num<'a> {
    #[inline]
    fn from(value: &'a mut i8) -> Self {
        Self::I8(value)
    }
}

impl<'a> From<&'a mut i16> for Num<'a> {
    #[inline]
    fn from(value: &'a mut i16) -> Self {
        Self::I16(value)
    }
}

impl<'a> From<&'a mut i32> for Num<'a> {
    #[inline]
    fn from(value: &'a mut i32) -> Self {
        Self::I32(value)
    }
}

impl<'a> From<&'a mut i64> for Num<'a> {
    #[inline]
    fn from(value: &'a mut i64) -> Self {
        Self::I64(value)
    }
}
