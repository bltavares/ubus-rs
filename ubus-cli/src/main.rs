use std::{
    ffi::CStr,
    path::Path,
    ptr,
    sync::mpsc::{self, Receiver},
};

fn main() {
    let ctx = UbusContext::connect();
    for item in ctx.list().iter() {
        println!("{:?}", item);
    }
    println!("{:?}", ctx);
    println!("Hello, world!");
}

#[derive(Debug)]
pub struct UbusContext {
    ctx: *mut ubus_sys::ubus_context,
}

fn path_to_pointer(path: &Path) -> *const ::std::os::raw::c_char {
    path.as_os_str().to_string_lossy().as_bytes().as_ptr() as *const _
}

impl UbusContext {
    pub fn connect() -> Self {
        Self::connect_to_path(None::<&str>)
    }

    pub fn connect_to_path<P: AsRef<Path>>(path: Option<P>) -> Self {
        let ctx = unsafe {
            ubus_sys::ubus_connect(
                path.map(|x| path_to_pointer(x.as_ref()))
                    .unwrap_or_else(ptr::null),
            )
        };
        if ctx.is_null() {
            panic!("TODO: Convert to Result");
        }

        Self { ctx }
    }

    pub fn list(&self) -> Receiver<UbusObjectData> {
        let (sender, receiver) = mpsc::channel();

        let callback = move |_, obj: *mut ubus_sys::ubus_object_data, _| {
            let path = unsafe { CStr::from_ptr((*obj).path) }
                .to_str()
                .expect("could not decode path")
                .to_owned();

            sender
                .send(UbusObjectData { path: dbg!(path) })
                .expect("could not send data");
        };
        let mut data = PrivateData {
            data: ptr::null_mut(),
            callback: Box::new(callback),
        };
        // TODO take path as argument
        let result = unsafe {
            ubus_sys::ubus_lookup(
                self.ctx,
                ptr::null(),
                Some(generic_callback),
                &mut data as *mut _ as *mut _,
            )
        };
        // TODO make it a Result
        assert_eq!(result, 0, "Failed operation");

        receiver
    }
}

#[derive(Debug)]
pub struct UbusObjectData {
    path: String,
}

type Callback = Box<
    dyn Fn(
        *mut ubus_sys::ubus_context,
        *mut ubus_sys::ubus_object_data,
        *mut ::std::os::raw::c_void,
    ),
>;

struct PrivateData {
    data: *mut ::std::os::raw::c_void,
    callback: Callback,
}

extern "C" fn generic_callback(
    ctx: *mut ubus_sys::ubus_context,
    obj: *mut ubus_sys::ubus_object_data,
    priv_: *mut ::std::os::raw::c_void,
) {
    let private = unsafe { &mut *(priv_ as *mut PrivateData) };
    (private.callback)(ctx, obj, private.data);
}

impl Drop for UbusContext {
    fn drop(&mut self) {
        unsafe { ubus_sys::ubus_free(self.ctx) };
    }
}
