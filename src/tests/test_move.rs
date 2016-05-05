#![allow(dead_code)]

struct C {
}

struct A {
    id: C
}

fn foo(e: &mut A) {
    let b = C{};
    e.id = b;
}

#[test]
fn test_move() {
    let n = C{};
    let mut c = A { id:  n};
    foo(&mut c);
}
