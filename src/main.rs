extern crate uxas;

use std::env::args;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

extern "C" {
    fn uxas_main(argc: c_int, argv: *mut (*mut c_char)) -> c_int;
}
    

fn main() {
    let mut raw_args: Vec<*mut c_char> = vec![];
    for arg in args() {
        raw_args.push(CString::new(arg).unwrap().into_raw());
    }
    unsafe {
        uxas_main(raw_args.len() as c_int, raw_args.as_mut_ptr());
    }
}
