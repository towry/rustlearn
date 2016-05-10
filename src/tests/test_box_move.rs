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

#[test]
fn test_box_move_again() {
    let b = get_box();
    let mut c: Vec<&Box<Boo>> = Vec::new();
    // c.push(b);
    c.push(&b);
    assert_eq!((*b).id, 3);
    let i = c[0];
    assert_eq!((*i).id, 3);
}
