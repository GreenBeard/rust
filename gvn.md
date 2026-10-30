// uh oh https://github.com/rust-lang/rust/issues/155884
//
// I probably should have read the linked issues in gvn.rs before starting...

use std::cell::UnsafeCell;

struct Foo<'a> {
    a: &'a u32,
    b: &'a mut u32,
    c: UnsafeCell<u32>,
    d: u32,
}

struct Bar<'a, 'b> {
    a: &'a Foo<'b>,
    b: &'a mut Foo<'b>,
    c: UnsafeCell<Foo<'b>>,
    d: Foo<'b>,
}

fn foo(foo: &Foo<'_>) {
    unsafe {
        (*foo);                      // no project
        (*foo).a;                    // no project since GVN isn't lifetime aware
        (*foo).b;                    // no project since GVN isn't lifetime aware
        (*foo).c;                    // no project
        (*foo).d;                    // project

        *(*foo).a;                   // no for now, see #155884
        *(*foo).b;                   // no for now, `&&mut` semantics currently unspecified
        *(*foo).c.get();             // no project
    }
}

fn bar(bar: &Bar<'_, '_>) {
    unsafe {
        (*bar);                      // no project
        (*bar).a;                    // no project since GVN isn't lifetime aware
        (*bar).b;                    // no project since GVN isn't lifetime aware
        (*bar).c;                    // no project
        (*bar).d;                    // no project

        *(*bar).a;                   // no project
        *(*bar).b;                   // no project
        *(*bar).c.get();             // no project

        (*(*bar).a).a;               // no project since GVN isn't lifetime aware
        (*(*bar).a).b;               // no project since GVN isn't lifetime aware
        (*(*bar).a).c;               // no project
        (*(*bar).a).d;               // no for now, see #155884

        (*(*bar).b).a;               // no project since GVN isn't lifetime aware
        (*(*bar).b).b;               // no project since GVN isn't lifetime aware
        (*(*bar).b).c;               // no project
        (*(*bar).b).d;               // no for now, see #155884

        (*(*bar).c.get()).a;         // no project
        (*(*bar).c.get()).b;         // no project
        (*(*bar).c.get()).c;         // no project
        (*(*bar).c.get()).d;         // no project

        (*bar).d.a;                  // no project since GVN isn't lifetime aware
        (*bar).d.b;                  // no project since GVN isn't lifetime aware
        (*bar).d.c;                  // no project
        (*bar).d.d;                  // project

        *(*(*bar).a).a;              // no for now, see #155884
        *(*(*bar).a).b;              // no for now, `&&mut` semantics currently unspecified & see #155884
        *(*(*bar).a).c.get();        // no project

        *(*(*bar).b).a;              // no for now, `&&mut` semantics currently unspecified & see #155884
        *(*(*bar).b).b;              // no for now, `&&mut` semantics currently unspecified
        *(*(*bar).b).c.get();        // no project

        *(*(*bar).c.get()).a;        // no project
        *(*(*bar).c.get()).b;        // no project
        *(*(*bar).c.get()).c.get();  // no project

        *(*bar).d.a;                 // no for now, see #155884
        *(*bar).d.b;                 // no for now, `&&mut` semantics currently unspecified
        *(*bar).d.c.get();           // no project
    }
}
