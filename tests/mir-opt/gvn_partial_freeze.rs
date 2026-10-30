//@ test-mir-pass: GVN
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// EMIT_MIR gvn_partial_freeze.cmp_freeze.GVN.diff
// EMIT_MIR gvn_partial_freeze.cmp_non_freeze.GVN.diff

use std::cell::UnsafeCell;

struct PartialFreeze {
    a: UnsafeCell<u32>,
    b: u32,
}

fn cmp_non_freeze(partial_freeze: &PartialFreeze, f: fn()) -> bool {
    // CHECK-LABEL: fn cmp_non_freeze(
    // CHECK: _0 = Eq(
    let old = unsafe { *partial_freeze.a.get() };
    f();
    let new = unsafe { *partial_freeze.a.get() };
    old == new
}

fn cmp_freeze(partial_freeze: &PartialFreeze, f: fn()) -> bool {
    // CHECK-LABEL: fn cmp_freeze(
    // CHECK: _0 = Eq(
    // to check: _0 = const true;
    let old = partial_freeze.b;
    f();
    let new = partial_freeze.b;
    old == new
}
