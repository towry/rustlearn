// Note, below tests is totally false.

#[test]
fn test_move_back() {
    let a = 3i32;
    let b = a;
    // can not use a here
    // Update on 1/9.2018: we can use a later, because a implemented the Copy
    // trait.

    // if we do not use let
    // it will alert the immutable error
    // this is not move back
    // it's redeclare the variable and move b
    // to it ...
    let a = b;
    assert_eq!(a, 3i32);
}

#[test]
fn test_move_back_with_mut() {
    let mut a = 3i32;
    let b = a;

    a = b;
    assert_eq!(a, 3i32);
    // so we can move it back!
}
