#![feature(freeze, freeze_impls, negative_impls)]

use std::marker::Freeze;

pub struct ManualFreeze {
    pub a: u32,
}

impl !Freeze for ManualFreeze {}

fn cmp_manual_freeze(partial_freeze: &ManualFreeze, f: fn()) -> bool {
    let old = partial_freeze.a;
    f();
    let new = partial_freeze.a;
    old == new
}
