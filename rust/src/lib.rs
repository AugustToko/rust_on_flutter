use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

///
/// network module for `lgj.wlwsx.client.flutter`
///

/* example */
#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_cstr_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return; }
        CString::from_raw(s)
    };
}

/* network */

#[no_mangle]
pub async extern fn net_test() -> *mut c_char {
    let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let response = client.post("http://httpbin.org/post")
        .form(&params)
        .send().await;

    return match response {
        Ok(res) => {
            let pre_text = res.text().await;

            let t2 = match pre_text {
                Ok(data) => {
                    CString::new(data).unwrap().into_raw()
                }
                Err(e2) => {
                    CString::new(e2.to_string()).unwrap().into_raw()
                }
            };
            t2
        }
        Err(err) => {
            CString::new(err.to_string()).unwrap().into_raw()
        }
    };
}
