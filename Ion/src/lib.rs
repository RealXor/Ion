#![feature(abi_thiscall)]
#![feature(core_intrinsics)]

/// Hello there, you're probably wonder how I got in this situation?
/// /s either way to allow the use of thiscall we need to have the nightly rust build (angery face)
/// so that's why :)

#[macro_use]
extern crate lazy_static;

/* our modules */
mod vmt;
mod utils;
mod ion;

use winapi::{
    shared::minwindef::{HMODULE, DWORD, BOOL, LPVOID, TRUE},
    um::{
        libloaderapi::{DisableThreadLibraryCalls},
        winnt::{DLL_PROCESS_ATTACH},
        processthreadsapi::{CreateThread},
        consoleapi::AllocConsole,
        wincon::SetConsoleTitleA,
    },
    ctypes::c_void,
};

unsafe extern "system" fn dllmain_wrapped(module: *mut c_void) -> u32
{
    let res = std::panic::catch_unwind(|| {
        /* allocate a console */
        AllocConsole();
        /* set epic console title */
        SetConsoleTitleA(b"Ion\0".as_ptr() as _);

        /* start epic cheat */
        ion::start();
    });

    match res {
        Err(e) => println!("Error: {:?}", e),
        _ => {}
    };

    0
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn DllMain(module: HMODULE, reason: DWORD, _: LPVOID) -> BOOL {
    /* Disable thread calls */
    DisableThreadLibraryCalls(module);

    /* If we're attaching, create a new thread */
    if reason == DLL_PROCESS_ATTACH {
        /* NOTE: leaking handle here, need to find a way to resolve */
        CreateThread(std::ptr::null_mut(), 0, Some(dllmain_wrapped), module as *mut _, 0,std::ptr::null_mut());
    }

    /* return true always */
    TRUE
}