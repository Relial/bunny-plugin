use std::{cell::SyncUnsafeCell, marker::PhantomData, mem::MaybeUninit, sync::Once};

// Unwise

/// A std::sync::LazyLock that lets you shoot yourself in the foot
pub struct HookCell<T: Sync> {
    once: Once,
    value: SyncUnsafeCell<MaybeUninit<T>>,
    _marker: PhantomData<T>,
}

impl<T: Sync> HookCell<T> {
    #[allow(clippy::new_without_default)]
    #[inline]
    pub const fn new() -> Self {
        Self {
            once: Once::new(),
            value: SyncUnsafeCell::new(MaybeUninit::uninit()),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn get(&self) -> Option<&T> {
        if self.initialized() {
            Some(unsafe { self.get_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    pub fn set(&self, value: T) -> Result<(), T> {
        match self.try_insert(value) {
            Ok(_) => Ok(()),
            Err((_, value)) => Err(value),
        }
    }

    #[inline]
    pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
        let mut value = Some(value);
        let res = self.get_or_init(|| value.take().unwrap());
        match value {
            None => Ok(res),
            Some(value) => Err((res, value)),
        }
    }

    #[inline]
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        if let Some(value) = self.get() {
            return value;
        }
        let slot = &self.value;
        self.once.call_once_force(|_| {
            let v = f();
            unsafe { (&mut *slot.get()).write(v) };
        });
        unsafe { self.get_unchecked() }
    }

    /// # Safety
    /// None
    #[inline]
    pub unsafe fn get_unchecked(&self) -> &T {
        unsafe { (&*self.value.get()).assume_init_ref() }
    }

    /// # Safety
    /// None
    #[allow(clippy::mut_from_ref)]
    #[inline]
    pub unsafe fn get_unchecked_mut(&self) -> &mut T {
        unsafe { (&mut *self.value.get()).assume_init_mut() }
    }

    /// # Safety
    /// None
    #[inline]
    pub unsafe fn drop(&self) {
        if self.initialized() {
            unsafe {
                (&mut *self.value.get()).assume_init_drop();
            };
        }
    }

    #[inline]
    fn initialized(&self) -> bool {
        self.once.is_completed()
    }
}
