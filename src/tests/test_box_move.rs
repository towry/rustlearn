#![allow(dead_code)]

struct Boo {
    id: i32
}

fn get_box() -> Box<Boo> {
    let b = Boo {
        id: 3
    };
    Box::new(b)
}

#[test]
fn test_box_move() {
    let b = get_box();
    assert_eq!((*b).id, 3);
}
