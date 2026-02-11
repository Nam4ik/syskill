use std::ffi::CString;
use std::os::raw::c_int;
use clap::{Parser, Subcommand};


mod non_critical;
mod critical;

#[derive(Parser)]
#[command(name = "suicidekit")]
#[command(about = "Toolkit: Critical and non-critical loads", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

extern "C" { fn check_root() -> i32; }

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
    KmodPanic, 
    SysRQPanic
}

fn main() {
    let cli: Cli = Parser::parse();

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
        Commands::GuiDestroyer => {
            let status = unsafe { check_root() };
            let _ = non_critical::gui_destroyer::artifacts_and_kill(status == 1, 100000);
        },
        Commands::StopGuiDestroyer => {
            let _ = non_critical::gui_destroyer::artifacts_and_kill(false, 0);
        },
        Commands::Wipe { drive, random } => unsafe {
            let c_drive = CString::new(drive).expect("CString");
            critical::wipe_with_dd(c_drive.as_ptr(), random as bool);
        },
        Commands::RemoveRoot => unsafe {
            critical::remove_root();
        },
        Commands::KmodPanic => {
            match non_critical::kern_panic::kmod_panic() {
                Ok(_) => println!("Kernel module loaded successfully"),
                Err(e) => eprintln!("Error loading kernel module: {}", e),
            }
        },
        Commands::SysRQPanic => unsafe {
            non_critical::kern_panic::sysrq_panic();
        },
    }
}


