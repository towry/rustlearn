use std::any::Any;

fn print_if_string(s: &Any) {
   let string = match s.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match s.downcast_ref::<String>() {
            Some(s) => {
                let c = String::from("abc");
                let d: () = c;
                &**s
            },
            None => "Box<Any>",
        }
   };
   println!("{}", string);
}

fn main() {
    // let a =
    // print_if_string(&0);
    print_if_string(&"cookie monster".to_string());
}
