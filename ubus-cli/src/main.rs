fn main() {
    let args = std::env::args().nth(1);

    let ctx = ubus::Context::connect();
    println!("{:?}", ctx);
    for item in ctx.list(args.as_deref()).iter() {
        println!("{:?}", item);
    }
    println!("Hello, world!");
}
