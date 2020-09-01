
extern crate rustlearn;

fn main() {
    rustlearn::tests::test_multiple_lifetime_specifier::test_multiple_lifetime_specifier();
}

/*

// 假如这里不声明生命周期参数，那么编译器无法知道我们的intent是什么。
// 我原来想不通的地方是，为什么需要多个生命周期参数，直接声明一个lifetime参数a，
// 用来说明其他参数的生命周期不能小于生命周期 a 不就行了吗？
// 比如下面的函数，我想改成 fn foo<'a>(x: &'a u32, y: &'a u32) -> &'a u32;
// 但是，在真正编译的时候，此函数 call site 的地方，实例化的作用域是不一样的，比如x的做用于是a, 传进去y参数的作用域
// 是b，并且 a > b。那么在调用的时候，就会报错 &y 的生命周期小于 'a 。我们的intent是两个参数的生命周期是一样的，
// 但是编译器在编译的时候，实例化生命周期参数的时候，发现传进去的生命周期和我们intent的对不上，编译器肯定要报错了。
// 这就是为啥要 显式 地声明生命周期。

fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'a u32 {
    x
}

fn main() {
    let x = 12;
    let z: &u32 = {
        let y = 42;
        // foo 返回的生命周期要大于等于 z 的生命周期.
        // 假如我们 foo(&y, &x) 那么 foo 返回的是 y，但是 y 的生命周期
        // 小于 z 的生命周期，但是 foo 函数声明中，写明返回值的生命周期必须
        // 大于等于 foo 的 call site 的生命周期（foo被调用时所处的scope）。
        foo(&x, &y)
    };
}

*/
