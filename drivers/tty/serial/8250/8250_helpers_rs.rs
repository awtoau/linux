// SPDX-License-Identifier: GPL-2.0
//! 8250/16550 pure register-bit-manipulation helper(s) — Rust translation
//! of a narrow slice of `drivers/tty/serial/8250/8250_port.c`.
//!
//! Scope: `serial8250_compute_lcr()` only (termios cflag -> LCR byte). This
//! is Tier A per docs/serial-8250-translation-scoping-2026-07-18.md: pure
//! arithmetic, zero register I/O, zero locking, zero interrupt-context
//! involvement — the lowest-risk possible slice of a driver that is
//! otherwise unusually high-stakes because it *is* the console this
//! project's own test harness reads to decide pass/fail (see that doc for
//! the full risk discussion). `fcr_get_rxtrig_bytes`/`bytes_to_fcr_rxtrig`
//! are deliberately NOT included here even though they were verified
//! alongside this function in `bench/diff_8250_helpers.rs` — they index
//! the driver-wide `uart_config[]` static table, which would need porting
//! in full to keep this a genuinely narrow, self-contained slice.
//!
//! Faithful port of the C original; oracle-verified byte-identical against
//! `bench/diff_8250_helpers.c` over 7500 generated cflag combinations via
//! `scripts/diff_oracle.py 8250_helpers` (see that doc, "Verification
//! gate"). `tty_get_char_size()` stays a real exported C symbol
//! (`drivers/tty/tty_ioctl.c`, `EXPORT_SYMBOL_GPL`) — declared here rather
//! than reimplemented, since duplicating it would defeat the point of a
//! narrow slice and it has no further translation-relevant dependencies.

use kernel::ffi::{c_uchar, c_uint};

const UART_LCR_SPAR: c_uchar = 0x20;
const UART_LCR_EPAR: c_uchar = 0x10;
const UART_LCR_PARITY: c_uchar = 0x08;
const UART_LCR_STOP: c_uchar = 0x04;

const CSTOPB: c_uint = 0x0000_0040;
const PARENB: c_uint = 0x0000_0100;
const PARODD: c_uint = 0x0000_0200;
const CMSPAR: c_uint = 0x4000_0000;

unsafe extern "C" {
    /// `drivers/tty/tty_ioctl.c:tty_get_char_size()` — real exported C
    /// symbol (`EXPORT_SYMBOL_GPL`), not translated (see module docs).
    fn tty_get_char_size(cflag: c_uint) -> c_uchar;
}

/// `include/linux/serial.h:UART_LCR_WLEN(x)`
#[inline]
fn uart_lcr_wlen(x: c_uchar) -> c_uchar {
    x - 5
}

/// `8250_port.c:serial8250_compute_lcr()`. The `up` (`struct
/// uart_8250_port *`) parameter of the C original is dropped: the C body
/// never dereferences it, only `c_cflag` is used (confirmed by reading the
/// original body; the sole caller passes `up` only so this function has
/// the same shape as its neighbours). The C-side wrapper left in
/// `8250_port.c` keeps the original signature so the call site is
/// untouched.
///
/// # Safety
/// `tty_get_char_size` is called with a plain `c_uint`, no pointers
/// involved; safe to call from any context the C original could be
/// called from.
///
/// Not `#[export]`: that macro requires a matching `kernel::bindings::`
/// entry (i.e. a C declaration bindgen already saw), which would mean
/// adding this driver-internal helper to `bindings_helper.h` — out of
/// proportion for a single non-public function called from exactly one
/// C file. Plain `#[no_mangle]` (same pattern as
/// `rust/kernel/iommu/pgtable.rs`) plus a hand-written `extern` decl on
/// the C side achieves the same C-ABI linkage without that.
#[no_mangle]
pub unsafe extern "C" fn serial8250_compute_lcr_rs(c_cflag: c_uint) -> c_uchar {
    // SAFETY: tty_get_char_size takes a plain integer and returns a plain
    // integer, no aliasing/lifetime concerns.
    let mut lcr = uart_lcr_wlen(unsafe { tty_get_char_size(c_cflag) });

    if c_cflag & CSTOPB != 0 {
        lcr |= UART_LCR_STOP;
    }
    if c_cflag & PARENB != 0 {
        lcr |= UART_LCR_PARITY;
    }
    if c_cflag & PARODD == 0 {
        lcr |= UART_LCR_EPAR;
    }
    if c_cflag & CMSPAR != 0 {
        lcr |= UART_LCR_SPAR;
    }

    lcr
}
