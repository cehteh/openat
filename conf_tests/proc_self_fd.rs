extern crate libc;

fn main() {
    unsafe {
        let conf_tests = std::ffi::CString::new("/proc/self/fd/0").unwrap();
        if libc::open(conf_tests.as_ptr(), libc::O_RDONLY) != -1 {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }
}
