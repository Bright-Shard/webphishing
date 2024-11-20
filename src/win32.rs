//! Opinionated Win32 bindings - the windows crate is pretty ass

use core::{
    ffi::{c_char, c_void},
    mem::{self, MaybeUninit},
    ptr,
};

#[repr(u32)]
#[derive(PartialEq, Eq, Debug)]
#[allow(dead_code)] // for completeness
pub enum DllEntryReason {
    ProcessAttach = 1,
    ProcessDetach = 0,
    ThreadAttach = 2,
    ThreadDetach = 3,
}

#[repr(transparent)]
pub struct Win32Bool(i32);
impl From<bool> for Win32Bool {
    fn from(value: bool) -> Self {
        if value {
            Self(1)
        } else {
            Self(0)
        }
    }
}
impl From<Win32Bool> for bool {
    fn from(value: Win32Bool) -> Self {
        value.0 != 0
    }
}

#[derive(Clone, Copy)]
pub struct Process(HProcess);
impl Process {
    pub fn current() -> Self {
        Self(GetCurrentProcess())
    }
}

#[derive(Clone, Copy)]
pub struct Module(HModule);
impl Module {
    pub fn current_executable() -> Self {
        Self(GetModuleHandleA(ptr::null()))
    }

    pub fn info(self, process: Process) -> ModuleInfo {
        let mut module_info = MaybeUninit::uninit();

        GetModuleInformation(
            process.0,
            self.0,
            module_info.as_mut_ptr(),
            mem::size_of::<ModuleInfo>() as u32,
        );

        unsafe { module_info.assume_init() }
    }
}

// Below here is raw FFI (kowai)

// Type notes:
// LPCWSTR = null-terminated 16-bit Unicode string
// LPCSTR = null-terminated ASCII string
// LPVOID = *mut c_void
// DWORD = u32

#[repr(transparent)]
#[derive(Clone, Copy)]
struct HModule(*mut c_void);

#[repr(transparent)]
#[derive(Clone, Copy)]
struct HProcess(*mut c_void);

#[repr(C)]
pub struct ModuleInfo {
    pub base_of_dll: *mut c_void,
    pub size_of_image: u32,
    pub entry_point: *mut c_void,
}

#[link(name = "kernel32")]
unsafe extern "C" {
    pub safe fn AllocConsole() -> Win32Bool;
    safe fn GetModuleHandleA(name: *const c_char) -> HModule;
    safe fn GetModuleInformation(
        process: HProcess,
        module: HModule,
        mod_info: *mut ModuleInfo,
        cb: u32,
    ) -> Win32Bool;
    safe fn GetCurrentProcess() -> HProcess;
}
