
# closure

http://huonw.github.io/blog/2015/05/finding-closure-in-rust/

```rs
fn make_adder(x: i32) -> Box<Fn(i32) -> i32> {
    // raise error, because use x in closure is a reference to the fn's x.
    // and x will go out of scope once this fn is end.
    Box::new(|y| x + y) // the x is from the closure struct.
}

fn main() {
    let f = make_adder(3);

    println!("{}", f(1)); // 4
    println!("{}", f(10)); // 13
}
```

```rs
// This is what the code look like to compiler.
// The struct is used to capture variables.
struct Closure<'a> {
    x: &'a i32
}

/* impl of Fn for Closure */
fn make_adder(x: i32) -> Box<Fn(i32) -> i32> {
    Box::new(Closure { x: &x })
}
```

Use move keyword.

```rs
fn make_adder(x: i32) -> Box<Fn(i32) -> i32> {
    // Since x will be dropped anyway, so we move it.
    // but x is Copyable, even we moved it, we just move a copy.
    // if x is not copyable, and you use it after the the move,
    // it will raise an error: use of moved value: `x.val`
    let ret = Box::new(move |y| x + y)
    // you can use x here, because x is copyable.
    println!("bbb {}", x);
    // return.
    ret
}

fn main() {
    let f = make_adder(3);
    println!("{}", f(1));
    println!("{}", f(10));
}
```

Use move keyword with non-copyable

```rs
struct Val {
    val: i32,
}

fn make_adder(x: Val) -> Box<Fn(i32) -> i32> {
    let a = Box::new(move |y| x.val + y);
    println!("ccc {}", a(3));
    // raise error, because x is moved to the closure environment.
    // captured by the closure.
    println!("bbb {}", x.val);
    a
}

fn main() {
    let v = Val { val: 3 };
    let f = make_adder(v);
    println!("{}", f(1));
    println!("{}", f(10));
}
```
