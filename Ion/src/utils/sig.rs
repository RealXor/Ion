use std::i32;
use std::str;

/**
    Signature utilities
*/

use winapi::{
    ctypes::{c_char, c_long},
    shared::minwindef::{DWORD, HMODULE},
    um::winnt::{PIMAGE_DOS_HEADER, PIMAGE_NT_HEADERS},
};

/// I'm tired, I'll fix this later
fn pattern_to_bytes(pattern: String) -> Vec<i32> {
    pattern.replace(' ', "").as_bytes()
        .chunks(2)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .into_iter()
        .filter(|&q| !q.contains('?'))
        .map(|q| { i32::from_str_radix(q, 16).unwrap() })
        .collect::<Vec<i32>>()
}

pub fn pattern_scan(module: HMODULE, sig: &str) -> *mut u8 {
    unsafe {
        let dos_headers = module as PIMAGE_DOS_HEADER;

        let module_addr = module as usize;
        let e_lfanew = (*dos_headers).e_lfanew as c_long;

        let nt_headers = (module_addr + e_lfanew as usize) as PIMAGE_NT_HEADERS;

        let size_of_image = (*nt_headers).OptionalHeader.SizeOfImage as usize;
        let pattern_bytes = pattern_to_bytes(sig.to_owned());
        let bytes = module as *mut u8;

        let size = pattern_bytes.len();

        for i in 0..(size_of_image - size as usize) {
            let mut found = true;
            for j in 0..size {
                if *bytes.offset(i as isize + j as isize) != pattern_bytes[j] as _ && pattern_bytes[j] != -1 {
                    found = false;
                    break;
                }
            }

            if found {
                return bytes.offset(i as _) as _;
            }
        }
    }

    0 as *mut _
}