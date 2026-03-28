use napi_derive::napi;

#[napi]
pub fn say_hello(name: String) -> String {
    format!("Hello, {name}")
}
