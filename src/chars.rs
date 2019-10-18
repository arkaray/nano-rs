#![crate_type = "staticlib"]

#[no_mangle]


pub extern "C" fn do_something() {
    println!("hello");
}