
#[macro_export]
macro_rules! macros_foo {
    (x + $e:expr) => (println!("mode x: {}", $e));
}


#[macro_export]
macro_rules! map {
    ( $( $key:expr => $value:expr ),* ) => ({
        let mut hm = ::std::collections::HashMap::new();
        $( hm.insert($key, $value); )*
        hm
    })
}


// https://github.com/steveklabnik/trpl/blob/master/macros.md
