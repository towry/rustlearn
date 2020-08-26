trait Foo {}

struct Bar {}

impl Foo for Bar {}

fn foo<T: Foo + ?Sized>(_y: &T) {}

fn main() {
  foo(&Bar {});
}
