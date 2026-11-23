//@ test-mir-pass: GVN
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// EMIT_MIR gvn_partial_ref.cmp_ref.GVN.diff
// EMIT_MIR gvn_partial_ref.cmp_non_ref.GVN.diff

struct PartialRef<'a> {
    a: &'a u32,
    b: u32,
}

fn cmp_non_ref(partial_ref: &PartialRef, f: fn()) -> bool {
    // CHECK-LABEL: fn cmp_non_ref(
    // CHECK: _0 = Eq(
    let old = *partial_ref.a;
    f();
    let new = *partial_ref.a;
    old == new
}

fn cmp_ref(partial_ref: &PartialRef, f: fn()) -> bool {
    // CHECK-LABEL: fn cmp_ref(
    // CHECK: _0 = Eq(
    // to check: _0 = const true;
    let old = partial_ref.b;
    f();
    let new = partial_ref.b;
    old == new
}
