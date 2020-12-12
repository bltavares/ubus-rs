use ubus_serde::UbusExtension;

fn main() {
    let args = std::env::args().nth(1);

    let ctx = ubus::Context::connect();
    println!("{:?}", ctx);
    for item in ctx.list(args.as_deref()).iter() {
        println!("{:?}", item);
    }
    println!(
        "{}",
        ctx.call("hostapd.wlan1", "get_clients").recv().unwrap()
    );
    println!(
        "{:?}",
        ctx.call_as::<Clients>("hostapd.wlan1", "get_clients")
            .recv()
            .unwrap()
    );
    println!("Hello, world!");
}

#[derive(serde::Deserialize, Debug)]
struct Clients {
    freq: u32,
}
