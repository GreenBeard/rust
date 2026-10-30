//@ test-mir-pass: GVN

use std::cell::UnsafeCell;

struct PartialFreeze {
    a: UnsafeCell<u32>,
    b: u32,
}

// EMIT_MIR gvn_partial_freeze.cmp_non_freeze.GVN.diff
fn cmp_non_freeze(partial_freeze: &PartialFreeze, f: fn()) -> bool {
    let old = unsafe { *partial_freeze.a.get() };
    f();
    let new = unsafe { *partial_freeze.a.get() };
    old == new
}

// EMIT_MIR gvn_partial_freeze.cmp_freeze.GVN.diff
fn cmp_freeze(partial_freeze: &PartialFreeze, f: fn()) -> bool {
    let old = partial_freeze.b;
    f();
    let new = partial_freeze.b;
    old == new
}
