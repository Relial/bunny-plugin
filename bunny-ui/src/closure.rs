use std::{marker::PhantomCovariantLifetime, ptr::NonNull};

use crate::{input::BunnyInputState, ui::BunnyUi};

#[repr(C)]
pub struct PluginClosure<'a> {
    closure: NonNull<u8>,
    closure_trampoline: unsafe extern "C" fn(&mut BunnyUi, NonNull<u8>),
    phantom: PhantomCovariantLifetime<'a>,
}

impl<'a> PluginClosure<'a> {
    #[inline]
    pub fn new<F: FnMut(&mut BunnyUi) + 'a>(closure: &mut F) -> Self {
        let closure = NonNull::from_mut(closure).cast::<u8>();
        Self {
            closure,
            closure_trampoline: closure_trampoline::<F>,
            phantom: PhantomCovariantLifetime::new(),
        }
    }

    #[inline]
    pub fn call(self, ui: &mut BunnyUi) {
        unsafe { (self.closure_trampoline)(ui, self.closure) }
    }
}

unsafe extern "C" fn closure_trampoline<F: FnMut(&mut BunnyUi)>(
    ui: &mut BunnyUi,
    closure: NonNull<u8>,
) {
    let closure = unsafe { closure.cast::<F>().as_mut() };
    closure(ui)
}

#[repr(C)]
pub struct InputStateClosure<'a> {
    closure: NonNull<u8>,
    closure_trampoline: unsafe extern "C" fn(&mut BunnyInputState, NonNull<u8>),
    phantom: PhantomCovariantLifetime<'a>,
}

impl<'a> InputStateClosure<'a> {
    #[inline]
    pub fn new<F: FnMut(&mut BunnyInputState) + 'a>(closure: &mut F) -> Self {
        let closure = NonNull::from_mut(closure).cast::<u8>();
        Self {
            closure,
            closure_trampoline: input_state_trampoline::<F>,
            phantom: PhantomCovariantLifetime::new(),
        }
    }

    #[inline]
    pub fn call(self, input_state: &mut BunnyInputState) {
        unsafe { (self.closure_trampoline)(input_state, self.closure) }
    }
}

unsafe extern "C" fn input_state_trampoline<F: FnMut(&mut BunnyInputState)>(
    input_state: &mut BunnyInputState,
    closure: NonNull<u8>,
) {
    let closure = unsafe { closure.cast::<F>().as_mut() };
    closure(input_state)
}
