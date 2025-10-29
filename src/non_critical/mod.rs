pub mod gui_destroyer;

use std::os::raw::{c_char, c_int};

extern "C" {
    // Tools    
    pub fn check_root() -> c_int;
    pub fn check_pid(pid: c_int) -> c_int;
    pub fn get_os_name() -> *mut c_char;

    // Random sounds
    pub fn init_random_sounds(threads: c_int, time: c_int) -> c_int;
    pub fn stop_random_sounds();
    
    // Syscall storm
    pub fn init_syscall_storm(threads: c_int, iterations: c_int) -> c_int;
    pub fn stop_syscall_storm();
}


