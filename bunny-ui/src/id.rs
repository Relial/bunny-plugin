use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Id(u64);

impl Id {
    pub const NULL: Self = Self::new(u64::MAX);

    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    #[inline]
    pub fn from_salt(salt: impl Hash) -> Self {
        let mut hasher = rapidhash::fast::RapidHasher::default_const();
        salt.hash(&mut hasher);
        Self(hasher.finish())
    }

    #[inline]
    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn hashed(&self) -> Self {
        let mut hasher = rapidhash::fast::RapidHasher::default_const();
        self.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn with(self, salt: impl Hash) -> Self {
        let mut hasher = rapidhash::fast::RapidHasher::default_const();
        self.hash(&mut hasher);
        salt.hash(&mut hasher);
        Self(hasher.finish())
    }
}

#[cfg(feature = "manager")]
impl From<Id> for egui::Id {
    fn from(value: Id) -> Self {
        Self::new(value)
    }
}
