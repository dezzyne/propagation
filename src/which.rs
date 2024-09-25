extern crate sys_info;

// pub mod which {
pub fn os() {
    // match sys_info::os_type() -> {

    // }
    println!("{}", sys_info::os_type().unwrap_or_default());
}
// }
