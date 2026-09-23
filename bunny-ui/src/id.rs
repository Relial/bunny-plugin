use std::{
    hash::{BuildHasher as _, Hash, Hasher as _},
    num::NonZeroU64,
};

use rapidhash::fast::GlobalState;

// reimplement this so plugins don't need egui as a dependency
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Id(NonZeroU64);

impl Id {
    #[inline]
    const fn from_hash(hash: u64) -> Self {
        if let Some(nonzero) = NonZeroU64::new(hash) {
            Self(nonzero)
        } else {
            Self(NonZeroU64::MIN)
        }
    }

    #[inline]
    pub fn new(source: impl Hash) -> Self {
        Self::from_hash(GlobalState::new().hash_one(source))
    }

    #[inline]
    pub fn with(self, salt: impl Hash) -> Self {
        let mut hasher = GlobalState::new().build_hasher();
        hasher.write_u64(self.0.get());
        salt.hash(&mut hasher);
        Self::from_hash(hasher.finish())
    }

    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.0.get()
    }
}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}", self.value() as u16)
    }
}

impl From<&'static str> for Id {
    #[inline]
    fn from(string: &'static str) -> Self {
        Self::new(string)
    }
}

impl From<String> for Id {
    #[inline]
    fn from(string: String) -> Self {
        Self::new(string)
    }
}

#[cfg(feature = "manager")]
impl From<Id> for egui::Id {
    #[inline]
    fn from(value: Id) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(feature = "manager")]
impl From<egui::Id> for Id {
    #[inline]
    fn from(value: egui::Id) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
