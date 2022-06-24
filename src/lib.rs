extern crate libc;

#[no_mangle]
pub extern "C" fn foo_rs(a: u32, b: u32) {
    println!("hello from rust : a + b = {}", a + b);
}
#[link(name = "foo", kind = "static")]
extern "C" {
    // this is rustified prototype of the function from our C library
    pub fn testcall(v: f32);
}

extern "C" {
    #[link(name = "foo++", kind = "static")]
    pub fn testcall_cpp(v: f32);
}
