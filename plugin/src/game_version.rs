use bitflags::bitflags;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameVersion(u32);

bitflags! {
    impl GameVersion: u32 {
        const ZZ = 1 << 0;
        const G32 = 1 << 1;
    }
}
