
use std::fs;

fn test<'a>() -> &'a str {
    println!("{}", &*format!("v{}", "hello"));
    "hello"
}

fn main() {
    println!("{}", test());

    let a = fs::File::open("abcde.txt");
    match a {
        Ok(_) => {
            println!("Ok");
        },
        Err(_) => {
            println!("err");
        }
    }
}
