use std::{marker::PhantomCovariantLifetime, mem::MaybeUninit, ptr::NonNull};

use crate::{input::BunnyInputState, ui::BunnyUi};

#[repr(C)]
struct OutputPointer(NonNull<u8>);

impl OutputPointer {
    #[inline]
    fn new<R>(output: &mut MaybeUninit<R>) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(output.as_mut_ptr()).cast::<u8>() };
        Self(ptr)
    }
}

#[repr(C)]
struct ClosurePointer(NonNull<u8>);

impl ClosurePointer {
    #[inline]
    fn new<R, F: FnMut(&mut BunnyUi) -> R>(closure: &mut F) -> Self {
        Self(NonNull::from_mut(closure).cast::<u8>())
    }
}

#[repr(C)]
pub struct PluginClosure<'a> {
    closure: ClosurePointer,
    closure_trampoline: unsafe extern "C" fn(&mut BunnyUi, ClosurePointer, OutputPointer),
    output: OutputPointer,
    phantom: PhantomCovariantLifetime<'a>,
}

impl<'a> PluginClosure<'a> {
    #[inline]
    pub fn new<R, F: FnMut(&mut BunnyUi) -> R + 'a>(
        closure: &mut F,
        output: &mut MaybeUninit<R>,
    ) -> Self {
        let closure = ClosurePointer::new(closure);
        let output = OutputPointer::new(output);
        Self {
            closure,
            closure_trampoline: closure_trampoline::<R, F>,
            output,
            phantom: PhantomCovariantLifetime::new(),
        }
    }

    #[inline]
    pub fn call(self, ui: &mut BunnyUi) {
        unsafe { (self.closure_trampoline)(ui, self.closure, self.output) }
    }
}

unsafe extern "C" fn closure_trampoline<R, F: FnMut(&mut BunnyUi) -> R>(
    ui: &mut BunnyUi,
    closure: ClosurePointer,
    output: OutputPointer,
) {
    let closure = unsafe { closure.0.cast::<F>().as_mut() };
    let ret = closure(ui);
    unsafe { output.0.cast::<R>().write(ret) };
}

#[repr(C)]
struct NoReturnClosurePointer(NonNull<u8>);

impl NoReturnClosurePointer {
    #[inline]
    fn new<F: FnMut(&mut BunnyUi)>(closure: &mut F) -> Self {
        Self(NonNull::from_mut(closure).cast::<u8>())
    }
}

#[repr(C)]
pub struct PluginNoReturnClosure<'a> {
    closure: NoReturnClosurePointer,
    closure_trampoline: unsafe extern "C" fn(&mut BunnyUi, NoReturnClosurePointer),
    phantom: PhantomCovariantLifetime<'a>,
}

impl<'a> PluginNoReturnClosure<'a> {
    #[inline]
    pub fn new<F: FnMut(&mut BunnyUi) + 'a>(closure: &mut F) -> Self {
        let closure = NoReturnClosurePointer::new(closure);
        Self {
            closure,
            closure_trampoline: closure_no_return_trampoline::<F>,
            phantom: PhantomCovariantLifetime::new(),
        }
    }

    #[inline]
    pub fn call(self, ui: &mut BunnyUi) {
        unsafe { (self.closure_trampoline)(ui, self.closure) }
    }
}

unsafe extern "C" fn closure_no_return_trampoline<F: FnMut(&mut BunnyUi)>(
    ui: &mut BunnyUi,
    closure: NoReturnClosurePointer,
) {
    let closure = unsafe { closure.0.cast::<F>().as_mut() };
    closure(ui)
}

#[repr(C)]
struct InputStateClosurePointer(NonNull<u8>);

impl InputStateClosurePointer {
    #[inline]
    fn new<R, F: FnMut(&mut BunnyInputState) -> R>(closure: &mut F) -> Self {
        Self(NonNull::from_mut(closure).cast::<u8>())
    }
}

#[repr(C)]
pub struct InputStateClosure<'a> {
    closure: InputStateClosurePointer,
    closure_trampoline:
        unsafe extern "C" fn(&mut BunnyInputState, InputStateClosurePointer, OutputPointer),
    output: OutputPointer,
    phantom: PhantomCovariantLifetime<'a>,
}

impl<'a> InputStateClosure<'a> {
    #[inline]
    pub fn new<R, F: FnMut(&mut BunnyInputState) -> R + 'a>(
        closure: &mut F,
        output: &mut MaybeUninit<R>,
    ) -> Self {
        let closure = InputStateClosurePointer::new(closure);
        let output = OutputPointer::new(output);
        Self {
            closure,
            closure_trampoline: input_state_trampoline::<R, F>,
            output,
            phantom: PhantomCovariantLifetime::new(),
        }
    }

    #[inline]
    pub fn call(self, input_state: &mut BunnyInputState) {
        unsafe { (self.closure_trampoline)(input_state, self.closure, self.output) }
    }
}

unsafe extern "C" fn input_state_trampoline<R, F: FnMut(&mut BunnyInputState) -> R>(
    input_state: &mut BunnyInputState,
    closure: InputStateClosurePointer,
    output: OutputPointer,
) {
    let closure = unsafe { closure.0.cast::<F>().as_mut() };
    let ret = closure(input_state);
    unsafe { output.0.cast::<R>().write(ret) };
}

#[repr(C)]
struct ScrollAreaRowsClosurePointer(NonNull<u8>);

impl ScrollAreaRowsClosurePointer {
    #[inline]
    fn new<R, F: FnMut(&mut BunnyUi, std::ops::Range<usize>) -> R>(closure: &mut F) -> Self {
        Self(NonNull::from_mut(closure).cast::<u8>())
    }
}

#[repr(C)]
pub struct ScrollAreaRowsClosure<'a> {
    closure: ScrollAreaRowsClosurePointer,
    closure_trampoline:
        unsafe extern "C" fn(&mut BunnyUi, &[usize; 2], ScrollAreaRowsClosurePointer, OutputPointer),
    output: OutputPointer,
    phantom: PhantomCovariantLifetime<'a>,
}

impl<'a> ScrollAreaRowsClosure<'a> {
    #[inline]
    pub fn new<R, F: FnMut(&mut BunnyUi, std::ops::Range<usize>) -> R + 'a>(
        closure: &mut F,
        output: &mut MaybeUninit<R>,
    ) -> Self {
        let closure = ScrollAreaRowsClosurePointer::new(closure);
        let output = OutputPointer::new(output);
        Self {
            closure,
            closure_trampoline: scroll_area_rows_closure_trampoline::<R, F>,
            output,
            phantom: PhantomCovariantLifetime::new(),
        }
    }

    #[inline]
    pub fn call(self, ui: &mut BunnyUi, range: &[usize; 2]) {
        unsafe { (self.closure_trampoline)(ui, range, self.closure, self.output) }
    }
}

unsafe extern "C" fn scroll_area_rows_closure_trampoline<
    R,
    F: FnMut(&mut BunnyUi, std::ops::Range<usize>) -> R,
>(
    ui: &mut BunnyUi,
    range: &[usize; 2],
    closure: ScrollAreaRowsClosurePointer,
    output: OutputPointer,
) {
    let closure = unsafe { closure.0.cast::<F>().as_mut() };
    let ret = closure(ui, range[0]..range[1]);
    unsafe { output.0.cast::<R>().write(ret) };
}
