use std::ffi::CString;
use std::os::raw::c_char;

use jni::objects::{JClass, JString};
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_rustlib_RustLib_stringFromJNI<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    let s = String::from("Hello from Rust! updated");
    let response = env.new_string(s).expect("Couldn't create java string!");
    // response.into_raw()
    response
}


pub type Callback = extern "C" fn(*const c_char);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn invokeCallbackViaJNA(callback: Callback) {
    let s = CString::new("Hello from Rust!").unwrap();
    callback(s.as_ptr());
}