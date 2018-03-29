use libc::getpid;
use uuid::Uuid;


pub fn pid() -> i32 {
    unsafe { getpid() as i32 }
}

pub fn nonce() -> String {
    Uuid::new_v4().to_string()
}
