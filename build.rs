fn main() {
    println!("cargo:rerun-if-changed=src/non_critical/_tools.c");
    println!("cargo:rerun-if-changed=src/non_critical/_tools.h");
    println!("cargo:rerun-if-changed=src/non_critical/random_sounds.c");
    println!("cargo:rerun-if-changed=src/non_critical/random_sounds.h");
    println!("cargo:rerun-if-changed=src/non_critical/syscall_storm.c");
    println!("cargo:rerun-if-changed=src/non_critical/syscall_storm.h");
    println!("cargo:rerun-if-changed=src/critical/classic/dd.c");
    println!("cargo:rerun-if-changed=src/critical/classic/dd.h");
    println!("cargo:rerun-if-changed=src/critical/classic/rm_root.c");
    println!("cargo:rerun-if-changed=src/critical/classic/rm_root.h");

    let mut build = cc::Build::new();
    build
        .warnings(false)
        .flag_if_supported("-std=c11")
        .include("src/non_critical")
        .include("src/critical/classic")
        .file("src/non_critical/_tools.c")
        .file("src/non_critical/random_sounds.c")
        .file("src/non_critical/syscall_storm.c")
        .file("src/critical/classic/dd.c")
        .file("src/critical/classic/rm_root.c");


    build.compile("libsuicidekit_c");


    #[cfg(target_os = "linux")]
    {

        println!("cargo:rustc-link-lib=asound");
        println!("cargo:rustc-link-lib=pthread");
    }
}


