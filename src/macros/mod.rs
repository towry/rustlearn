
#[macro_export]
macro_rules! macros_foo {
    (x + $e:expr) => (println!("mode x: {}", $e));
}
