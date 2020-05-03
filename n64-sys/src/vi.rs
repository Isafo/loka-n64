#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};
use n64_math::Color;
use n64_types::VideoMode;

const VI_BASE: usize = 0xA440_0000;

const VI_STATUS: *mut usize = VI_BASE as _;
const VI_DRAM_ADDR: *mut usize = (VI_BASE + 0x04) as _;
const VI_H_WIDTH: *mut usize = (VI_BASE + 0x08) as _;
const VI_V_INTR: *mut usize = (VI_BASE + 0x0C) as _;
const VI_CURRENT: *const usize = (VI_BASE + 0x10) as _;
const VI_TIMING: *mut usize = (VI_BASE + 0x14) as _;
const VI_V_SYNC: *mut usize = (VI_BASE + 0x18) as _;
const VI_H_SYNC: *mut usize = (VI_BASE + 0x1C) as _;
const VI_H_SYNC_LEAP: *mut usize = (VI_BASE + 0x20) as _;
const VI_H_VIDEO: *mut usize = (VI_BASE + 0x24) as _;
const VI_V_VIDEO: *mut usize = (VI_BASE + 0x28) as _;
const VI_V_BURST: *mut usize = (VI_BASE + 0x2C) as _;
const VI_X_SCALE: *mut usize = (VI_BASE + 0x30) as _;
const VI_Y_SCALE: *mut usize = (VI_BASE + 0x34) as _;

static mut LAST_BUFFER: Option<*mut Color> = None;

#[inline]
pub fn init(video_mode: VideoMode, fb: &mut [Color]) {
    match video_mode {
        VideoMode::Ntsc { .. } => unsafe {
            write_volatile(VI_STATUS, 0x0000_320E);
            write_volatile(VI_DRAM_ADDR, fb.as_mut_ptr() as usize);
            write_volatile(VI_H_WIDTH, video_mode.width() as usize);
            write_volatile(VI_V_INTR, 2);
            write_volatile(VI_TIMING, 0x03E5_2239);
            write_volatile(VI_V_SYNC, 0x0000_020D);
            write_volatile(VI_H_SYNC, 0x0000_0C15);
            write_volatile(VI_H_SYNC_LEAP, 0x0C15_0C15);
            write_volatile(VI_H_VIDEO, 0x006C_02EC);
            write_volatile(VI_V_VIDEO, 0x0025_01FF);
            write_volatile(VI_V_BURST, 0x000E_0204);
            write_volatile(VI_X_SCALE, 0x100 * video_mode.width() as usize / 160);
            write_volatile(VI_Y_SCALE, 0x100 * video_mode.height() as usize / 60);
        },
        VideoMode::Pal { .. } => unsafe {
            write_volatile(VI_STATUS, 0x0000_320E);
            write_volatile(VI_DRAM_ADDR, fb.as_mut_ptr() as usize);
            write_volatile(VI_H_WIDTH, video_mode.width() as usize);
            write_volatile(VI_V_INTR, 0x200);
            write_volatile(VI_TIMING, 0x0040_4233A);
            write_volatile(VI_V_SYNC, 0x0000_0271);
            write_volatile(VI_H_SYNC, 0x0015_0C69);
            write_volatile(VI_H_SYNC_LEAP, 0x0C6F_0C6E);
            write_volatile(VI_H_VIDEO, 0x0080_0300);
            write_volatile(VI_V_VIDEO, 0x005F_0239);
            write_volatile(VI_V_BURST, 0x0009_026B);
            write_volatile(VI_X_SCALE, 0x100 * video_mode.width() as usize / 160);
            write_volatile(VI_Y_SCALE, 0x100 * video_mode.height() as usize / 60);
        },
    }
}

#[inline]
pub fn wait_for_vblank() {
    loop {
        let current_halfline = unsafe { read_volatile(VI_CURRENT) };
        if current_halfline <= 1 {
            break;
        }
    }
}

#[inline]
pub fn set_vi_buffer(fb: &mut [Color]) {
    unsafe {
        LAST_BUFFER = Some(fb.as_mut_ptr());
        write_volatile(VI_DRAM_ADDR, fb.as_mut_ptr() as usize);
    }
}

#[inline]
pub fn get_vi_buffer() -> *mut Color {
    unsafe { LAST_BUFFER.unwrap() }
}
