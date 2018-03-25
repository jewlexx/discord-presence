use libc::getpid;

pub fn pid() -> i32 {
    unsafe { getpid() as i32 }
}
