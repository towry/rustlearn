
fn test<'a>() -> &'a str {
    println!("{}", &*format!("v{}", "hello"));
    "hello"
}

fn main() {
    println!("{}", test());
}
