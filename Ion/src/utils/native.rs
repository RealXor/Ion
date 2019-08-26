/**
    Some simple wrappers
*/

use winapi::{
    ctypes::c_void,
    shared::minwindef::HMODULE,
    um::libloaderapi::{GetModuleHandleA, GetProcAddress},
};

pub fn get_module_handle(name: *const u8) -> HMODULE {
    unsafe { return GetModuleHandleA(name as _) }
}

pub fn get_proc_address(module: HMODULE, name: *const u8) -> *const c_void {
    unsafe { return GetProcAddress(module, name as _) as _}
}

pub fn get_virtual_function(base: *mut usize, idx: isize) -> *mut usize {
    unsafe {
        /* credits to cyanidee on github */
        let vt = *base as *mut usize;
        let vfn = vt.offset(idx).read() as *mut usize;

        return vfn;
    }
}
