use jni::objects::{JClass, JString};
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_rustlib_RustLib_stringFromJNI<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    let s = String::from("Hello from Rust!");
    let response = env.new_string(s).expect("Couldn't create java string!");
    response
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_rustlib_RustLib_inputFun<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    let s: String = env.get_string(&input).expect("Couldn't get java string!").into();
    let response = env.new_string(format!("Hi {}, this is Rust", s)).expect("Couldn't create java string!");
    response
}