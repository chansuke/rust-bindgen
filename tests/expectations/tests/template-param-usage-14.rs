/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BaseIgnoresT {
    pub x: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CrtpIgnoresU {
    pub _base: BaseIgnoresT,
    pub y: ::std::os::raw::c_int,
}
impl Default for CrtpIgnoresU {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
