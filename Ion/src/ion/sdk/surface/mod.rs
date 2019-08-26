
use crate::utils;
use std::os::raw::{c_int};

pub enum EFontFlags
{
    FontflagNone,
    FontflagItalic = 0x001,
    FontflagUnderline = 0x002,
    FontflagStrikeout = 0x004,
    FontflagSymbol = 0x008,
    FontflagAntialias = 0x010,
    FontflagGaussianblur = 0x020,
    FontflagRotary = 0x040,
    FontflagDropshadow = 0x080,
    FontflagAdditive = 0x100,
    FontflagOutline = 0x200,
    FontflagCustom = 0x400,
    FontflagBitmap = 0x800,
}

#[repr(C)]
pub struct Color {
    r: c_int,
    g: c_int,
    b: c_int,
    a: c_int,
}

impl Color {
    pub fn new_rgba(r: i32, g: i32, b: i32, a: i32) -> Self {
        Self {
            r,g,b,a,
        }
    }

    pub fn new_rgb(r: i32, g: i32, b: i32) -> Self {
        Self {
            r,g,b, a: 255,
        }
    }
}

type set_drawing_color_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, r: i32, g: i32, b: i32, a: i32);
type draw_filled_rect_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, x: i32, y: i32, x1: i32, y1: i32);
type set_text_font_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, font: u32);
type set_text_color_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, r: i32, color: Color);
type set_text_pos_fn =  unsafe extern "thiscall" fn(thisptr: *mut usize, x: i32, y: i32);
type draw_print_text_fn =  unsafe extern "thiscall" fn(thisptr: *mut usize, text: *const u16, len: i32);

#[derive(Debug)]
pub struct c_surface {
    pub base: *mut usize,
}

impl c_surface {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn set_draw_color(&self, color: Color) {
        let vfunc = unsafe { std::mem::transmute::<_, set_drawing_color_fn>(utils::native::get_virtual_function(self.base, 15))};
        unsafe { vfunc(self.base, color.r, color.g, color.b, color.a); }
    }

    pub fn draw_filled_rect(&self, x: i32, y: i32, x1: i32, y1: i32) {
        let vfunc = unsafe { std::mem::transmute::<_, draw_filled_rect_fn>(utils::native::get_virtual_function(self.base, 16))};
        unsafe { vfunc(self.base, x, y, x1, y1); }
    }
}

impl Default for c_surface {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}

