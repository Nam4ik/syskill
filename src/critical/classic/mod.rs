use std::os::raw::c_char;

extern "C" {
    pub fn random_data_linux(sym_drive: *const c_char);
    pub fn rm_root();
}

pub unsafe fn wipe_with_dd(sym_drive: *const c_char) {
    random_data_linux(sym_drive);
}

pub unsafe fn remove_root() {
    rm_root();
}
