use uuid::Uuid;


#[allow(unused)]
pub fn pid() -> u32 {
    std::process::id()
}

pub fn nonce() -> String {
    Uuid::new_v4().to_string()
}
