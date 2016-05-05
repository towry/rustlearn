
struct A {
    a: Vec<i32>
}

fn foo(b: &A) -> &i32 {
    // item must be a reference to the item
    // in b.a
    // because the item in b.a can not move
    // to item
    for item in b.a.iter() {
        return item;
    }
    return &b.a[1];
}

#[test]
fn test_foo() {
    let a = A {
        a: vec!(1, 2, 3)
    };

    let c = foo(&a);
    println!("{}", c);
}
