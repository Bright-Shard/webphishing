use {
    crate::{
        log,
        win32::{Module, Process},
    },
    retour::RawDetour,
    std::{ffi::c_void, mem, ops::Deref, sync::OnceLock},
};

const IDA_BASE: usize = 0x140000000;

pub static HOOKS: OnceLock<Hooks> = OnceLock::new();

pub struct Hook {
    /// The actual detour.
    pub detour: RawDetour,
    /// The address of the original function.
    pub original: usize,
    /// The address of the new function.
    pub new: usize,
}
impl Deref for Hook {
    type Target = RawDetour;

    fn deref(&self) -> &Self::Target {
        &self.detour
    }
}
impl Hook {
    pub fn new(original: usize, new: usize) -> Self {
        Self {
            detour: unsafe { RawDetour::new(original as _, new as _) }.unwrap(),
            original,
            new,
        }
    }
}

pub struct Hooks {
    /// ResourceLoader::load
    resource_loader_load: Hook,
}
impl Hooks {
    pub fn load() -> Self {
        let executable = Module::current_executable();
        let info = executable.info(Process::current());
        let base = info.base_of_dll as usize;

        let vaddr = move |addr: usize| base + (addr - IDA_BASE);

        let target = vaddr(0x00000001409719C0);
        log!("Hooking 0x{target:x}");

        Self {
            resource_loader_load: Hook::new(target, resource_loader_load as _),
        }
    }
    pub fn enable(&self) {
        unsafe { self.resource_loader_load.enable().unwrap() }
    }
    pub fn is_enabled(&self) -> bool {
        self.resource_loader_load.is_enabled()
    }
}

extern "C" fn resource_loader_load(
    path: *const c_void,
    type_hint: *const c_void,
    cache_mode: u8,
    error: *mut c_void,
) {
    log!("detour");

    let hook = &HOOKS.get().unwrap().resource_loader_load;
    unsafe { hook.disable().unwrap() };

    let orig: extern "C" fn(*const c_void, *const c_void, u8, *mut c_void) =
        unsafe { mem::transmute(hook.original) };
    orig(path, type_hint, cache_mode, error);
}
