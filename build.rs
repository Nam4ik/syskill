use std::env;
use std::path::PathBuf;

fn main() {
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    println!("cargo:rerun-if-changed={}", src_dir.join("src/non_critical/_tools.c").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/non_critical/_tools.h").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/non_critical/random_sounds.c").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/non_critical/random_sounds.h").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/non_critical/syscall_storm.c").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/non_critical/syscall_storm.h").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/critical/classic/dd.c").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/critical/classic/dd.h").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/critical/classic/rm_root.c").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/critical/classic/rm_root.h").display());
    println!("cargo:rerun-if-changed={}", src_dir.join("src/critical/fork_bomb.c").display());

    let mut build = cc::Build::new();
    build
        .warnings(false)
        .flag_if_supported("-std=c11")
        .flag_if_supported("-O3")
        .include(src_dir.join("src/non_critical"))
        .include(src_dir.join("src/critical/classic"))
        .file(src_dir.join("src/non_critical/_tools.c"))
        .file(src_dir.join("src/non_critical/random_sounds.c"))
        .file(src_dir.join("src/non_critical/syscall_storm.c"))
        .file(src_dir.join("src/critical/classic/dd.c"))
        .file(src_dir.join("src/critical/classic/rm_root.c"))
        .file(src_dir.join("src/critical/fork_bomb.c"));

    build.compile("suicidekit_c");

    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=suicidekit_c");

   
    println!("cargo:rustc-link-lib=asound");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=pthread");
}