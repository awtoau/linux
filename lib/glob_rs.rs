// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//! Shell-style glob(7) pattern matching — Rust translation of
//! `lib/glob.c`.
//!
//! `glob_match_str`'s C original is a non-recursive backtracking state
//! machine using `goto` to merge two distinct control paths into shared
//! `literal:`/`backtrack:` logic:
//!   - `default:` (plain literal char, or `\`-escaped char) falls
//!     through into `literal:` naturally.
//!   - the `'['` (character class) arm does `goto literal` on a
//!     malformed class (bare `\0` or dangling `a-` with no upper
//!     bound) — treating the still-unconsumed `[` as a literal char
//!     to match against `c` (`d` is unchanged on this path, still `[`,
//!     matching the C which never reassigns `d` here).
//!   - the `'['` arm does `goto backtrack` on a class match/inverted
//!     mismatch.
//! An attempt to factor `literal`/`backtrack` into shared closures hit
//! a genuine borrow-checker conflict (both close over `pat`/`str_` by
//! mutable reference, but the surrounding loop also mutates them
//! around each call site) — not a superficial issue, a real structural
//! mismatch between C's unstructured goto and Rust's borrow rules for
//! this particular shape. Inlined the (short) literal/backtrack logic
//! at each of the 3 real call sites instead: `default:`'s fallthrough,
//! `'['`'s malformed-class goto, and `\`'s escaped-char case (which
//! also falls into `literal:` in the C, via `fallthrough;`). Each copy
//! is identical by construction — verified against the original C by
//! direct side-by-side re-read, not just "it compiles."
//!
//! `#[no_mangle]` directly (not `#[export]`): `glob.h`'s declarations
//! aren't in `rust/bindings/bindings_helper.h`, same gap noted in
//! `lib/ctype_rs.rs`/`lib/hexdump_rs.rs`/`lib/zstd/common/error_private_rs.rs`.

use kernel::ffi::c_char;

/// C: `glob_match_str` (`lib/glob.c`) — the shared backtracking core
/// behind both `glob_match`/`glob_match_len`. `str_end` is `None` for
/// the NUL-terminated case, `Some(ptr)` for the length-bounded case
/// (the one-past-the-end pointer, matching the C's `str + len`).
///
/// # Safety
/// `pat` must be a valid NUL-terminated string. `str_` must be valid
/// to read up to (and including) its first NUL byte, or — if
/// `str_end` is `Some(end)` — up to `end` (exclusive), whichever comes
/// first (a NUL byte within that range still terminates the string,
/// matching the C's `str_end && str >= str_end` early check).
unsafe fn glob_match_str(
    pat: *const c_char,
    str_: *const c_char,
    str_end: Option<*const c_char>,
) -> bool {
    // Backtrack to previous * on mismatch and retry starting one
    // character later in the string. Because * matches all characters
    // (no exception for /), it can be easily proved that there's
    // never a need to backtrack multiple levels.
    let mut back_pat: Option<*const c_char> = None;
    let mut back_str: Option<*const c_char> = None;

    let mut pat = pat;
    let mut str_ = str_;

    // Loop over each token (character or class) in pat, matching it
    // against the remaining unmatched tail of str. Return false on
    // mismatch, or true after matching the trailing nul bytes.
    loop {
        let at_end = str_end.is_some_and(|end| str_ >= end);
        // SAFETY: caller contract — `str_` valid to read here (either
        // genuinely before the NUL terminator, or within `str_end`'s
        // bound); `pat` valid to read and advance per caller contract.
        let c: u8 = if at_end { 0 } else { unsafe { *str_.cast::<u8>() } };
        let d: u8 = unsafe { *pat.cast::<u8>() };
        pat = unsafe { pat.add(1) };
        str_ = unsafe { str_.add(1) };

        match d {
            b'?' => {
                // Wildcard: anything but nul.
                if c == 0 {
                    return false;
                }
            }
            b'*' => {
                // Any-length wildcard.
                // SAFETY: `pat` valid to read (caller contract).
                if unsafe { *pat.cast::<u8>() } == 0 {
                    // Optimize trailing * case.
                    return true;
                }
                back_pat = Some(pat);
                str_ = unsafe { str_.sub(1) }; // Allow zero-length match.
                back_str = Some(str_);
            }
            b'[' => {
                // Character class.
                if c == 0 {
                    return false; // No possible match.
                }
                let mut is_match = false;
                // SAFETY: `pat` valid to read (caller contract).
                let inverted = unsafe { *pat.cast::<u8>() } == b'!';
                let mut class = if inverted { unsafe { pat.add(1) } } else { pat };
                // SAFETY: `class` valid to read (caller contract; a
                // malformed class is handled below by checking `a == 0`
                // before any further dereference of `class`).
                let mut a: u8 = unsafe { *class.cast::<u8>() };
                class = unsafe { class.add(1) };

                // Iterate over each span in the character class. A
                // span is either a single character a, or a range
                // a-b. The first span may begin with ']'.
                let mut malformed = false;
                loop {
                    let mut b = a;

                    if a == 0 {
                        malformed = true; // goto literal
                        break;
                    }

                    // SAFETY: `class` valid to read up to 2 bytes
                    // ahead per caller contract (NUL-terminated pat).
                    let class0 = unsafe { *class.cast::<u8>() };
                    if class0 == b'-' {
                        let class1 = unsafe { *class.add(1).cast::<u8>() };
                        if class1 != b']' {
                            b = class1;
                            if b == 0 {
                                malformed = true; // goto literal
                                break;
                            }
                            class = unsafe { class.add(2) };
                            // Any special action if a > b?
                        }
                    }
                    if a <= c && c <= b {
                        is_match = true;
                    }

                    // SAFETY: `class` valid to read (caller contract).
                    a = unsafe { *class.cast::<u8>() };
                    class = unsafe { class.add(1) };
                    if a == b']' {
                        break;
                    }
                }

                if malformed {
                    // `goto literal` — `d` is still `[` here, matching
                    // the C (it never reassigns `d` on this path).
                    // Inlined literal/backtrack (see module doc).
                    if c == d {
                        if d == 0 {
                            return true;
                        }
                        continue;
                    }
                    if c == 0 || back_pat.is_none() {
                        return false;
                    }
                    pat = back_pat.unwrap();
                    str_ = unsafe { back_str.unwrap().add(1) };
                    back_str = Some(str_);
                    continue;
                }

                if is_match == inverted {
                    // `goto backtrack`.
                    if c == 0 || back_pat.is_none() {
                        return false;
                    }
                    pat = back_pat.unwrap();
                    str_ = unsafe { back_str.unwrap().add(1) };
                    back_str = Some(str_);
                    continue;
                }
                pat = class;
            }
            b'\\' => {
                // SAFETY: `pat` valid to read (caller contract).
                let d = unsafe { *pat.cast::<u8>() };
                pat = unsafe { pat.add(1) };
                // `fallthrough;` into `literal:` in the C. Inlined
                // literal/backtrack (see module doc) — identical to
                // the `_ =>` arm below and the malformed-class copy
                // above by construction.
                if c == d {
                    if d == 0 {
                        return true;
                    }
                    continue;
                }
                if c == 0 || back_pat.is_none() {
                    return false;
                }
                pat = back_pat.unwrap();
                str_ = unsafe { back_str.unwrap().add(1) };
                back_str = Some(str_);
            }
            _ => {
                // Literal character. `literal:` — the natural
                // fallthrough case (see module doc).
                if c == d {
                    if d == 0 {
                        return true;
                    }
                    continue;
                }
                if c == 0 || back_pat.is_none() {
                    return false;
                }
                pat = back_pat.unwrap();
                str_ = unsafe { back_str.unwrap().add(1) };
                back_str = Some(str_);
            }
        }
    }
}

/// C: `glob_match` (`lib/glob.c`). Shell-style pattern matching, like
/// `!fnmatch(pat, str, 0)`. See the C original's doc comment for the
/// full pattern-syntax description (kept verbatim there, not
/// duplicated here).
#[no_mangle]
pub unsafe extern "C" fn glob_match(pat: *const c_char, str_: *const c_char) -> bool {
    // SAFETY: caller contract — both `pat` and `str_` NUL-terminated,
    // matching glob_match_str's contract with `str_end = None`.
    unsafe { glob_match_str(pat, str_, None) }
}

/// C: `glob_match_len` (`lib/glob.c`). Like `glob_match`, but `str` is
/// only read up to `len` bytes, so it can be used on buffers that are
/// not NUL-terminated (e.g. trace event fields). A NUL byte within
/// `len` still terminates the string.
#[no_mangle]
pub unsafe extern "C" fn glob_match_len(
    pat: *const c_char,
    str_: *const c_char,
    len: usize,
) -> bool {
    // SAFETY: caller contract — `pat` NUL-terminated, `str_` valid to
    // read up to `len` bytes.
    unsafe { glob_match_str(pat, str_, Some(str_.add(len))) }
}
