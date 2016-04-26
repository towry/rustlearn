
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
