#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Id(u32);

impl Id {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

#[cfg(feature = "manager")]
impl From<Id> for egui::Id {
    fn from(value: Id) -> Self {
        Self::new(value)
    }
}
