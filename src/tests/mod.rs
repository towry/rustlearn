
mod test_foo;
mod test_move_back;

#[cfg(target_os="linux")]
pub fn test_cfg() {
    println!("I am on linux");
}

#[cfg(not(target_os="linux"))]
pub fn test_cfg() {
    println!("I am on macos");
}

pub fn test_me() {
    println!("test me");
}

struct SomeThing {
    name: i32
}

pub fn test_mut() {
    // let i: &mut SomeThing = &mut SomeThing{ name: 3 };
    // or 
    let mut i: SomeThing = SomeThing{ name: 3 };
    i.name += 3;
    println!("{}", i.name);
}

pub fn test_raw_pointer() {
    let my_num = 10i32;
    let my_num_ptr: *const i32 = &my_num;
    println!("{:p}", my_num_ptr);
    unsafe {
        println!("{:?}", *my_num_ptr);
    }
}
