use std::process::Command;
use std::path::PathBuf;
use std::env;

extern "C" {
    pub fn linux_sysrq_start();
    pub fn linux_sysrq_panic();
}

pub unsafe fn sysrq_panic() {
    linux_sysrq_start();
    linux_sysrq_panic();
}

pub fn kmod_panic() -> Result<(), String> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")
        .map_err(|_| "OUT_DIR environment variable not set".to_string())?);
    let ko_file = out_dir.join("linux_kmod.ko");

    if !ko_file.exists() {
        return Err(format!("Kernel module not found at: {}", ko_file.display()));
    }

    let status = Command::new("insmod")
        .arg(&ko_file)
        .status()
        .map_err(|e| format!("Failed to execute insmod: {}", e))?;

    if !status.success() {
        return Err(format!("Failed to load kernel module: exit code {:?}", status.code()));
    }

    Ok(())
}

