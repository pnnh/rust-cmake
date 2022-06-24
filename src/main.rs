use foo_rs::{testcall, testcall_cpp};

fn main() {
    println!("Hello, world from Rust!");

    // calling the function from foo library
    unsafe {
        testcall(3.14159);
    };
    unsafe {
        testcall_cpp(3.14159);
    };
}
