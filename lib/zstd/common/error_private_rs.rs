// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
//! zstd error-code-to-string table — Rust translation of
//! `lib/zstd/common/error_private.c`.
//!
//! `ERR_enum` (`error_private.h`) is a typedef alias of the public
//! `ZSTD_ErrorCode` enum (`include/linux/zstd_errors.h`) — a plain C
//! `enum` with no explicit underlying type, so C gives it at least
//! `int`. Taken here as `c_int` and matched by value rather than
//! assumed `#[repr(C)]`-compatible with a Rust enum, since the C side
//! never specified a fixed-width representation.
//!
//! `ZSTD_STRIP_ERROR_STRINGS` is unset in this kernel's `.config`
//! (only `CONFIG_ZSTD_COMMON=y`/`CONFIG_ZSTD_DECOMPRESS=y`), so the
//! real string table below is the live path, not the stripped one-liner.

use kernel::c_str;
use kernel::ffi::{c_char, c_int};
use kernel::prelude::*;

const NOT_ERROR_CODE: &CStr = c_str!("Unspecified error code");

/// C: `ERR_getErrorString` (`error_private.c`), reached via
/// `ZSTD_getErrorString` (`zstd_common.c`, untranslated, calls this by
/// its real linked symbol — same `#[no_mangle]` pattern as
/// `lib/ctype_rs.rs`/`lib/hexdump_rs.rs`'s statics, extended to a
/// function: no bindgen `#[export]` typecheck needed since `ZSTD_ErrorCode`
/// isn't yet in `rust/bindings/bindings_helper.h`, and this matches the
/// real C ABI (`const char *(int)`) exactly).
#[no_mangle]
pub extern "C" fn ERR_getErrorString(code: c_int) -> *const c_char {
    let s: &CStr = match code {
        0 => c_str!("No error detected"),
        1 => c_str!("Error (generic)"),
        10 => c_str!("Unknown frame descriptor"),
        12 => c_str!("Version not supported"),
        14 => c_str!("Unsupported frame parameter"),
        16 => c_str!("Frame requires too much memory for decoding"),
        20 => c_str!("Data corruption detected"),
        22 => c_str!("Restored data doesn't match checksum"),
        24 => c_str!("Header of Literals' block doesn't respect format specification"),
        40 => c_str!("Unsupported parameter"),
        41 => c_str!("Unsupported combination of parameters"),
        42 => c_str!("Parameter is out of bound"),
        44 => c_str!("tableLog requires too much memory : unsupported"),
        46 => c_str!("Unsupported max Symbol Value : too large"),
        48 => c_str!("Specified maxSymbolValue is too small"),
        49 => c_str!("This mode cannot generate an uncompressed block"),
        50 => c_str!("pledged buffer stability condition is not respected"),
        30 => c_str!("Dictionary is corrupted"),
        32 => c_str!("Dictionary mismatch"),
        34 => c_str!("Cannot create Dictionary from provided samples"),
        70 => c_str!("Destination buffer is too small"),
        72 => c_str!("Src size is incorrect"),
        74 => c_str!("Operation on NULL destination buffer"),
        80 => c_str!("Operation made no progress over multiple calls, due to output buffer being full"),
        82 => c_str!("Operation made no progress over multiple calls, due to input being empty"),
        // following error codes are not stable and may be removed or changed in a future version
        100 => c_str!("Frame index is too large"),
        102 => c_str!("An I/O error occurred when reading/seeking"),
        104 => c_str!("Destination buffer is wrong"),
        105 => c_str!("Source buffer is wrong"),
        106 => c_str!("Block-level external sequence producer returned an error code"),
        107 => c_str!("External sequences are not valid"),
        60 => c_str!("Operation not authorized at current processing stage"),
        62 => c_str!("Context should be init first"),
        64 => c_str!("Allocation error : not enough memory"),
        66 => c_str!("workSpace buffer is not large enough"),
        _ => NOT_ERROR_CODE,
    };
    s.as_char_ptr()
}
