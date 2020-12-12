use std::sync::mpsc::{self, Receiver};

pub trait UbusExtension {
    fn call_as<T>(&self, path: &str, method: &str) -> Receiver<T>
    where
        T: serde::de::DeserializeOwned;
}

impl UbusExtension for ubus::Context {
    fn call_as<T>(&self, path: &str, method: &str) -> Receiver<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let (sender, receiver) = mpsc::channel();
        let json = self.call(path, method);

        for data in json.iter() {
            sender
                .send(serde_json::from_str(&data).expect("invalid json"))
                .expect("could not send message");
        }
        receiver
    }
}
