use ilhook::{
    HookError,
    x86::{CallbackOption, HookFlags, HookPoint, HookType, Hooker},
};

/// An ilhook hook builder that doesn't support CallbackOptions, and so implements Send and Sync for stuffing into a static
pub struct NoCbHookBuilder(Hooker);

impl NoCbHookBuilder {
    pub fn new(addr: usize, hook_type: HookType) -> Self {
        let builder = Hooker::new(addr, hook_type, CallbackOption::None, 0, HookFlags::empty());
        Self(builder)
    }

    /// # Safety
    /// None
    pub unsafe fn hook(self) -> Result<NoCbHookPoint, HookError> {
        let hook_point = unsafe { self.0.hook() }?;
        Ok(NoCbHookPoint(hook_point))
    }
}

#[allow(dead_code)]
pub struct NoCbHookPoint(HookPoint);

unsafe impl Send for NoCbHookPoint {}
unsafe impl Sync for NoCbHookPoint {}
