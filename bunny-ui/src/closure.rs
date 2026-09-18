use std::{ffi::c_void, marker::PhantomCovariantLifetime};

use crate::{input::BunnyInputState, ui::BunnyUi};

#[repr(C)]
pub struct PluginClosure<'a> {
    closure: *mut c_void,
    closure_trampoline: unsafe extern "C" fn(&mut BunnyUi, *mut c_void),
    phantom: PhantomCovariantLifetime<'a>,
}

impl<'a> PluginClosure<'a> {
    #[inline]
    pub fn new<F: FnMut(&mut BunnyUi) + 'a>(closure: &mut F) -> Self {
        let closure = closure as *mut F as *mut c_void;
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
    closure: *mut c_void,
) {
    let closure = unsafe { &mut *(closure as *mut F) };
    closure(ui)
}

#[repr(C)]
pub struct InputStateClosure<'a> {
    closure: *mut c_void,
    closure_trampoline: unsafe extern "C" fn(&mut BunnyInputState, *mut c_void),
    phantom: PhantomCovariantLifetime<'a>,
}

impl<'a> InputStateClosure<'a> {
    #[inline]
    pub fn new<F: FnMut(&mut BunnyInputState) + 'a>(closure: &mut F) -> Self {
        let closure = closure as *mut F as *mut c_void;
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
    closure: *mut c_void,
) {
    let closure = unsafe { &mut *(closure as *mut F) };
    closure(input_state)
}
