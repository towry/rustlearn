
mod test_foo;

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
