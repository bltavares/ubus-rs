fn main() {
    unsafe {
        let ctx = ubus_sys::ubus_connect(std::ptr::null());
        println!("{:?}", ctx);
    }
    println!("Hello, world!");
}
