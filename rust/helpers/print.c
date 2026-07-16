// SPDX-License-Identifier: GPL-2.0

#include <linux/printk.h>

/*
 * pr_debug()/pr_warn() are config-dependent macros (pr_debug compiles to
 * nothing unless DEBUG/dynamic-debug is in play; both need a module-
 * identity prefix normally set once via module!{}) — rule 0014. A
 * a per-file translated TU has no crate-root __LOG_PREFIX to
 * give kernel::print's pr_debug!/pr_warn! macros, so the exact C macro
 * call is shimmed here instead. One shim per distinct format string
 * (C varargs macros can't be wrapped generically across FFI either way).
 */
__rust_helper void rust_helper_pr_debug_decompress_magic(unsigned char b0, unsigned char b1)
{
	pr_debug("Compressed data magic: %#.2x %#.2x\n", b0, b1);
}

__rust_helper void rust_helper_pr_warn_cpio_name_too_long(const char *p, int max)
{
	pr_warn("File %s exceeding MAX_CPIO_FILE_NAME [%d]\n", p, max);
}
