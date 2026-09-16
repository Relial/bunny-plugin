use std::hash::{BuildHasher, Hash};

use rapidhash::fast::GlobalState;

#[inline]
pub fn hash_id_salt(id_salt: impl Hash) -> u64 {
    let hasher = GlobalState::new();
    hasher.hash_one(id_salt)
}
