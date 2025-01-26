fn main() {
    println!("cargo:rustc-link-search=native=/home/astrviktor/otus-rust/homework_15/smart_home/socket_test_rust/");
    println!("cargo:rustc-link-lib=dylib=socket");
}
