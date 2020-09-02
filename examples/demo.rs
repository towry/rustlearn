// #[allow(dead_code)]

// trait Foo {}

// #[derive(Debug)]
// struct F;
// #[derive(Debug)]
// struct Bar;

// impl Foo for F {}

// fn foo<T>(a1: &Bar, a2: &T) where T: std::fmt::Debug + Foo {
//   println!("{:?}, {:?}", a1, a2)
// }

// fn bar<'a>(a1: &'a Bar, a2: &'a Bar) -> &'a Bar {
//   return a2;
// }

// fn main() {
//   let a = Bar {};
//   {
//     let b = F {};
//     {
//       foo(bar(&a, &a), &b);
//     }
//   }
// }


fn foo<'a>(x: &'a u32, _y: &'a u32) -> &'a u32 {
  x
}

fn main() {
  let x = 12;
  let _z: &u32 = {
    let y = 42;
    foo(&x, &y)
  };
}
