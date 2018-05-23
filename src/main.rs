use std::env;
use std::os::raw::{c_char, c_int};

fn main() {
    let args: Vec<_> = env::args().collect();
    let c_argv: Vec<_> = args.iter()
        .map(|arg| arg.as_ptr())
        .collect();
    unsafe {
        uxas_main(c_argv.len() as c_int, c_argv.as_ptr() as *const *const c_char);
    }
}

extern "C" {
    fn uxas_main(argc: c_int, argv: *const *const c_char) -> c_int;
}
