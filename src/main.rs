use std::ffi::CString;
use std::os::raw::c_int;
use clap::{Parser, Subcommand};

mod non_critical;
mod critical;

#[path = "critical/classic/mod.rs"]
mod critical_classic;

#[derive(Parser)]
#[command(name = "suicidekit")]
#[command(about = "Toolkit: Critical and non-critical loads", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    RandomSounds {
        threads: i32,
        time: i32,
    },
    StopRandomSounds,
    SyscallStorm {
        threads: i32,
        iterations: i32,
    },
    StopSyscallStorm,
    Wipe {
        drive: String,
        random: bool,
    },
    RemoveRoot,
    ForkBomb,
    GuiDestroyer,
    StopGuiDestroyer,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::RandomSounds { threads, time } => unsafe {
            let _ = non_critical::init_random_sounds(threads as c_int, time as c_int);
        },
        Commands::StopRandomSounds => unsafe {
            non_critical::stop_random_sounds();
        },
        Commands::SyscallStorm { threads, iterations } => unsafe {
            let _ = non_critical::init_syscall_storm(threads as c_int, iterations as c_int);
        },
        Commands::StopSyscallStorm => unsafe {
            non_critical::stop_syscall_storm();
        },
        Commands::ForkBomb => unsafe {
            critical::fork_bomb();
        },
        Commands::GuiDestroyer => unsafe {
            non_critical::gui_destroyer();
        },
        Commands::StopGuiDestroyer => unsafe {
            non_critical::stop_gui_destroyer();
        },
        Commands::Wipe { drive, random } => unsafe {
            let c_drive = CString::new(drive).expect("CString");
            critical::wipe_with_dd(c_drive.as_ptr(), random as bool);
        },
        Commands::RemoveRoot => unsafe {
            critical::remove_root();
        },
    }
}


