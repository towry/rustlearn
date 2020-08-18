
use std::fs;

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn test<'a>() -> &'a str {
    println!("{}", format!("v{}", "hello"));
    "hello"
}

fn main() {
    println!("{}", test());

    let a = fs::File::open("abcde.txt");
    println!("{}", type_of(&a));
    match a {
        Ok(_) => {
            println!("Ok");
        },
        Err(_) => {
            println!("err");
        }
    }
}
