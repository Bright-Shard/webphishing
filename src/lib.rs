use {
    core::ffi::c_void,
    hooks::Hooks,
    std::{
        fs::{File, OpenOptions},
        panic::PanicHookInfo,
    },
    win32::{DllEntryReason, Win32Bool},
};

mod hooks;
mod win32;

fn open_log() -> File {
    OpenOptions::new()
        .create(true)
        .truncate(false)
        .append(true)
        .open("webphishing.log")
        .unwrap()
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        println!("[webphishing] {}", format_args!($($t)*));
        std::io::Write::write_fmt(
            &mut $crate::open_log(),
            format_args!("{}\n", format_args!($($t)*))
        ).unwrap();
    };
}

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(
    _dll: *mut c_void,
    reason: DllEntryReason,
    _reserved: *const c_void,
) -> Win32Bool {
    if reason == DllEntryReason::ProcessAttach {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("webphishing.log")
            .unwrap();

        win32::AllocConsole();
        log!("T-testing... testing, one, two, three?");
        log!("Webphishing loading...");

        log!("Setting panic handler");
        std::panic::set_hook(Box::new(panic_handler));
        log!("Loaded panic handler");

        log!("Loading Godot hooks");
        hooks::HOOKS.get_or_init(Hooks::load);
        log!("Godot hooks loaded");
        hooks::HOOKS.get().unwrap().enable();
        assert!(hooks::HOOKS.get().unwrap().is_enabled());
        log!("Godot hooks enabled");

        log!("Webphishing loaded.");
    }

    Win32Bool::from(true)
}

fn panic_handler(info: &PanicHookInfo<'_>) {
    log!("\n\n\n[webphishing] Panicked : {info}\n\n\n");
}
