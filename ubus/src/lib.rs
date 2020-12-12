use std::{
    ffi::{CStr, CString},
    path::Path,
    ptr,
    sync::mpsc::{self, Receiver},
};

#[derive(Debug)]
pub struct Context {
    ctx: *mut ubus_sys::ubus_context,
}

fn path_to_pointer(path: &Path) -> *const ::std::os::raw::c_char {
    path.as_os_str().to_string_lossy().as_bytes().as_ptr() as *const _
}

impl Context {
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

    pub fn list(&self, path: Option<&str>) -> Receiver<ObjectData> {
        let (sender, receiver) = mpsc::channel();
        let mut callback: ListCallback =
            Box::new(move |_, obj: *mut ubus_sys::ubus_object_data| {
                let path = unsafe { CStr::from_ptr((*obj).path) }
                    .to_str()
                    .expect("could not decode path")
                    .to_owned();
                let id = unsafe { (*obj).id };

                sender
                    .send(ObjectData { id, path })
                    .expect("could not send data");
            });

        let path = path.and_then(|x| CString::new(x).ok());

        // TODO take path as argument
        let result = unsafe {
            ubus_sys::ubus_lookup(
                self.ctx,
                path.as_deref().map(CStr::as_ptr).unwrap_or_else(ptr::null) as *const _,
                Some(list_callback),
                &mut callback as *mut _ as *mut _,
            )
        };
        // TODO make it a Result
        assert_eq!(result, 0, "Failed operation");

        receiver
    }

    // TODO take call arguments
    pub fn call(&self, path: &str, method: &str) -> Receiver<String> {
        let path = CString::new(path).unwrap();
        let mut id = 0;

        unsafe {
            ubus_sys::ubus_lookup_id(self.ctx, path.as_ptr() as *const _, &mut id);
        }
        assert_ne!(id, 0, "id not found");

        let method = CString::new(method).unwrap();
        let (sender, receiver) = mpsc::channel();

        let mut callback: InvokeCallback = Box::new(move |_, _, data| {
            let extracted = unsafe {
                ubus_sys::blobmsg_format_json_with_cb(data, true, None, ptr::null_mut(), -1)
            };
            let json = unsafe { CStr::from_ptr(extracted) }
                .to_string_lossy()
                .to_string();
            unsafe { libc::free(extracted as *mut _) };
            sender.send(json).expect("could not send json");
        });

        let result = unsafe {
            ubus_sys::ubus_invoke_fd(
                self.ctx,
                id,
                method.as_ptr() as *const _,
                ptr::null_mut(),
                Some(invoke_callback),
                &mut callback as *mut _ as *mut _,
                500,
                -1,
            )
        };
        assert_eq!(result, 0, "invoke didn't work");
        receiver
    }
}

#[derive(Debug)]
pub struct ObjectData {
    pub path: String,
    pub id: u32,
}

type ListCallback = Box<dyn Fn(*mut ubus_sys::ubus_context, *mut ubus_sys::ubus_object_data)>;

extern "C" fn list_callback(
    ctx: *mut ubus_sys::ubus_context,
    obj: *mut ubus_sys::ubus_object_data,
    priv_: *mut ::std::os::raw::c_void,
) {
    let callback = unsafe { &mut *(priv_ as *mut ListCallback) };
    (callback)(ctx, obj);
}

type InvokeCallback = Box<dyn Fn(*mut ubus_sys::ubus_request, i32, *mut ubus_sys::blob_attr)>;

extern "C" fn invoke_callback(
    request: *mut ubus_sys::ubus_request,
    typ: i32,
    message: *mut ubus_sys::blob_attr,
) {
    let callback = unsafe { &mut *((*request).priv_ as *mut InvokeCallback) };
    (callback)(request, typ, message);
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ubus_sys::ubus_free(self.ctx) };
    }
}
