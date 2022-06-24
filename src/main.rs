#[link(name = "foo", kind = "static")]
extern "C" {
    // this is rustified prototype of the function from our C library
    fn testcall(v: f32);
}

extern "C" {
    #[link(name = "foo++", kind = "static")]
    fn testcall_cpp(v: f32);
}

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
