use surf::StatusCode;


extern "C" {
    fn get_integer() -> i32;
}

fn main() {
    println!("Static lib integer: {}", unsafe { get_integer() });
    println!("Executable integer: {}", unsafe { get_integer() });
    println!("Surf code: {}", StatusCode::Accepted);
}

pub extern "C" fn get_integer_exe() -> i32 {
    if cfg!(target_feature = "crt-static") {
        1
    } else {
        2
    }
}
