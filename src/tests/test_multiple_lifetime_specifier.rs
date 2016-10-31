#![allow(dead_code)]
#![allow(unused_variables)]

// a struct with two
// different lifetime specifier
struct A<'a, 'b> {
    a: &'a i32,
    b: &'b i32
}


// a struct with same 
// lifetime specifier
// this means field a and b has the same lifetime
struct B<'a> {
    a: &'a i32,
    b: &'a i32
}

// #[test]
pub fn test_multiple_lifetime_specifier() {
    let outer_number = 1i32;

    let var_a = {
        let inner_number1 = 2;
        let local_a = A{a: &outer_number, b: &inner_number1};
        // let local_a = A{a: &outer_number, b: &outer_number};
        // this is a return
        local_a.a
    };

    // println!("Ok: {}", var_a);

    let outer_number2 = 3;
    // error
    let var_b = {
        let inner_number2 = 3;
        // below will cause an error.
        // let local_b = B{a: &outer_number2, b: &inner_number2};
        let local_b = B{a: &outer_number2, b: &outer_number2};
        local_b.a
        // println!("test in the inner: {}", local_b.a);
    };
    // println!("this will not print out: {}", var_b);
}
