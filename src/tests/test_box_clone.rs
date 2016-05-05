#![allow(dead_code)]

#[test]
fn test_box_clone() {
    let a = Box::new(3);
    assert_eq!(*a, 3);
    let d = a.clone();
    assert_eq!(*d, 3);
}
