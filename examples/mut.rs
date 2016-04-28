extern crate rustlearn;

fn main() {
    rustlearn::tests::test_mut();
}


#[allow(dead_code)]
fn foo(b: &mut i32) {
    // b now is a reference that point to some where in mem
    *b = 5;
    println!("{}", b);
}

// fn main() {
    // let mut a = 4i32;
    // not work
    // foo(&a);
    
    // work
//     foo(&mut a);
//     println!("{}", a);
// }
