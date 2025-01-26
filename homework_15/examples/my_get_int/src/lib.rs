#[no_mangle]
pub extern "C" fn get_integer() -> i32 {
    if cfg!(target_feature = "crt-static") {
        1
    } else {
        2
    }
}
