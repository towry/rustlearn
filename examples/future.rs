use futures::executor::block_on;

async fn get_message() -> String {
  String::from("hello world")
}

async fn hello_world() {
  println!("{}", get_message().await);
}

fn main() {
  let future = hello_world();
  block_on(future);
}
