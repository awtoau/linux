// SPDX-License-Identifier: GPL-2.0

#include <linux/printk.h>

/*
 * pr_debug() is a config-dependent macro (compiles to nothing unless
 * DEBUG is defined for this TU, or dynamic debug is in play) — rule
 * 0014. The kernel crate's own pr_debug! requires a crate-root
 * __LOG_PREFIX (set once via module!{}), which a per-file lib/*_rs.rs
 * translation has no access to; shimming the exact C macro call here
 * keeps the config-correct compiled-to-nothing-by-default behaviour
 * without requiring every translated TU to become its own "module".
 * One shim per distinct format string, same convention as other
 * rust_helper_* primitives (e.g. llist's per-operand try_cmpxchg shims)
 * — not a general varargs pr_debug (C macros can't be wrapped generically
 * across the FFI boundary either way).
 */
__rust_helper void rust_helper_pr_debug_decompress_magic(unsigned char b0, unsigned char b1)
{
	pr_debug("Compressed data magic: %#.2x %#.2x\n", b0, b1);
}
