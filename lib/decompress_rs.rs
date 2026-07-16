// SPDX-License-Identifier: GPL-2.0
//! Detect the decompression method based on magic number — Rust
//! translation of `lib/decompress.c`.
//!
//! Every `CONFIG_DECOMPRESS_*` option is unset in our target config, so
//! every table entry's decompressor slot is a NULL function pointer here,
//! exactly as it is in the C (`#define gunzip NULL` etc.) — this is not
//! dead code to skip, the table STRUCTURE and magic-number matching are
//! live and exercised; only the individual decompressor symbols are
//! absent for this config.
//!
//! `__init`/`__initconst` (freed after boot) expressed via
//! `#[link_section]` matching the sections `include/asm-generic/
//! vmlinux.lds.h` reserves for them — the closest faithful Rust
//! equivalent; the kernel crate has no higher-level wrapper for this yet
//! (checked 2026-07-16, no precedent found).

use kernel::bindings;
use kernel::ffi::{c_char, c_long, c_uchar, c_ulong};
use kernel::prelude::*;

type DecompressFn = Option<
    unsafe extern "C" fn(
        *mut c_uchar,
        c_long,
        Option<unsafe extern "C" fn(*mut kernel::ffi::c_void, c_ulong) -> c_long>,
        Option<unsafe extern "C" fn(*mut kernel::ffi::c_void, c_ulong) -> c_long>,
        *mut c_uchar,
        *mut c_long,
        Option<unsafe extern "C" fn(*mut c_char)>,
    ) -> kernel::ffi::c_int,
>;

#[repr(C)]
struct CompressFormat {
    magic: [u8; 2],
    name: *const c_char,
    decompressor: DecompressFn,
}
// SAFETY: a `*const c_char` to a 'static string literal is Sync (never
// mutated); this table is never written after initialisation, matching
// the C `static const` + __initconst placement.
unsafe impl Sync for CompressFormat {}

macro_rules! fmt {
    ($m0:expr, $m1:expr, $name:expr) => {
        CompressFormat {
            magic: [$m0, $m1],
            name: concat!($name, "\0").as_ptr().cast(),
            decompressor: None, // every CONFIG_DECOMPRESS_* is unset for this target
        }
    };
}

#[link_section = ".init.rodata"]
static COMPRESSED_FORMATS: [CompressFormat; 9] = [
    fmt!(0x1f, 0x8b, "gzip"),
    fmt!(0x1f, 0x9e, "gzip"),
    fmt!(0x42, 0x5a, "bzip2"),
    fmt!(0x5d, 0x00, "lzma"),
    fmt!(0xfd, 0x37, "xz"),
    fmt!(0x89, 0x4c, "lzo"),
    fmt!(0x02, 0x21, "lz4"),
    fmt!(0x28, 0xb5, "zstd"),
    // sentinel: name == NULL terminates the C original's `cf->name` loop
    CompressFormat { magic: [0, 0], name: core::ptr::null(), decompressor: None },
];

/// Detect the decompressor for `inbuf`/`len` by magic number.
///
/// # Safety
/// `inbuf` points to `len` valid bytes when `len >= 2`; `name`, if
/// non-NULL, is a valid `*mut *const c_char` to write the format name into.
#[link_section = ".init.text"]
#[export]
pub unsafe extern "C" fn decompress_method(
    inbuf: *const c_uchar,
    len: c_long,
    name: *mut *const c_char,
) -> DecompressFn {
    if len < 2 {
        // SAFETY: caller contract (name is a valid out-pointer if non-NULL).
        if !name.is_null() {
            unsafe { *name = core::ptr::null() };
        }
        return None; // Need at least this much...
    }

    // SAFETY: len >= 2 checked above; caller guarantees `len` valid bytes.
    let (b0, b1) = unsafe { (*inbuf, *inbuf.add(1)) };
    // SAFETY: pr_debug has no preconditions; the shim just formats+logs.
    unsafe { bindings::pr_debug_decompress_magic(b0, b1) };

    let cf = COMPRESSED_FORMATS
        .iter()
        .find(|cf| !cf.name.is_null() && cf.magic == [b0, b1])
        .unwrap_or_else(|| COMPRESSED_FORMATS.last().unwrap()); // sentinel: name == NULL

    // SAFETY: caller contract.
    if !name.is_null() {
        unsafe { *name = cf.name };
    }
    cf.decompressor
}
