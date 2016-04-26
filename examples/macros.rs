
#[macro_use(macros_foo)]
extern crate rustlearn;


fn main() {
    macros_foo!(x + 3);
}
