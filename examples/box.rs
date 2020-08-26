trait Animal {
  fn make_noise(&self) -> ();
}

struct Dog {}
struct Cat {}

impl Animal for Dog {
  fn make_noise(&self) -> () {
    println!("I am dog");
  }
}
impl Animal for Cat {
  fn make_noise(&self) -> () {
    println!("I am cat");
  }
}

fn main() {
  let dog: Box<Dog> = Box::new(Dog {});
  let cat: Box<Cat> = Box::new(Cat {});
  let mut pet: Box<dyn Animal>;
  pet = dog;
  pet.make_noise();
  pet = cat;
  pet.make_noise();
}
