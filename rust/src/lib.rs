use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use tokio::runtime::Runtime;
use std::io::prelude::*;
use std::net::TcpStream;

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
pub extern fn net_test() -> *mut c_char {
    let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let response = client.post("https://httpbin.org/post")
        .form(&params)
        .send();

    let rt = Runtime::new().unwrap();

    return match rt.block_on(response) {
        Ok(res) => {
            let pre_text = rt.block_on(res.text());

            let data = match pre_text {
                Ok(data) => {
                    CString::new(data).unwrap().into_raw()
                }
                Err(e2) => {
                    CString::new(e2.to_string()).unwrap().into_raw()
                }
            };
            data
        }
        Err(err) => {
            CString::new(err.to_string()).unwrap().into_raw()
        }
    };
}

fn arr_to_dec(m: &[u8; 8]) -> u8 {
    let mut sum: u8 = 0;
    let mut a: u8 = 1;  // 2的0次方

    let mut index = 1;
    while index != m.len() {
        if index > 0 {
            a = a * 2;
        }
        sum += a * m[index];
        index += 1;
    }

    return sum;
}

#[no_mangle]
pub extern fn modbus_write(ip: CString, data: &[u8; 8]) {
    if let Ok(mut stream) = TcpStream::connect(ip.to_str().unwrap() + ":502") {
        println!("Connected to the server!");
        let dec = arr_to_dec(data);
        let list = [0, 0, 0, 0, 0, 8, 1, 15, 0, 0, 0, 8, 1, dec];
        stream.write_all(&list).unwrap();
        stream.flush().unwrap();
    } else {
        println!("Couldn't connect to server...");
    }
}
