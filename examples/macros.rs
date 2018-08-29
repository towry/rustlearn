
#[macro_use(macros_foo, map)]
extern crate rustlearn;

fn main() {
    macros_foo!(x + 3);

    let user = map!(
        "name" => "Finn",
        "gender" => "Boy"
    );

    println!("User {:?}", user);
}
