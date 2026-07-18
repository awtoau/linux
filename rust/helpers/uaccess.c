// SPDX-License-Identifier: GPL-2.0

#include <linux/uaccess.h>

__rust_helper unsigned long
rust_helper_copy_from_user(void *to, const void __user *from, unsigned long n)
{
	return copy_from_user(to, from, n);
}

__rust_helper unsigned long
rust_helper_copy_to_user(void __user *to, const void *from, unsigned long n)
{
	return copy_to_user(to, from, n);
}

#ifdef INLINE_COPY_USER
__rust_helper
unsigned long rust_helper__copy_from_user(void *to, const void __user *from, unsigned long n)
{
	return _inline_copy_from_user(to, from, n);
}

__rust_helper
unsigned long rust_helper__copy_to_user(void __user *to, const void *from, unsigned long n)
{
	return _inline_copy_to_user(to, from, n);
}
#endif

/*
 * lib/strnlen_user.c support (rule 0014: access_ok/untagged_addr/
 * TASK_SIZE_MAX/user_read_access_begin/unsafe_get_user are all either
 * config- or CPU-feature-dependent macros -- riscv's untagged_addr()
 * reads current->mm and probes RISCV_ISA_EXT_SUPM at runtime; TASK_SIZE
 * is is_compat_task()-conditional under CONFIG_COMPAT -- shimmed rather
 * than reimplemented so this arch-specific, security-relevant logic
 * stays exactly as the real kernel compiles it, never re-derived.
 */

__rust_helper unsigned long rust_helper_strnlen_user_task_size_max(void)
{
	return TASK_SIZE_MAX;
}

__rust_helper unsigned long rust_helper_untagged_addr_ul(unsigned long addr)
{
	return (unsigned long)untagged_addr((void __user *)addr);
}

__rust_helper bool rust_helper_access_ok(const void __user *addr, unsigned long size)
{
	return access_ok(addr, size);
}

__rust_helper bool rust_helper_user_read_access_begin(const void __user *ptr, unsigned long len)
{
	return user_read_access_begin(ptr, len);
}

__rust_helper void rust_helper_user_read_access_end(void)
{
	user_read_access_end();
}

/*
 * Wraps unsafe_get_user() for a single `unsigned long` word. Must only
 * be called between rust_helper_user_read_access_begin()/_end() (mirrors
 * the C macro's own contract: it does not itself call access_ok() or
 * enable SUM, both of which are the *_begin() call's job).
 *
 * On a real page fault, the CPU traps into the kernel's exception
 * handler, which consults the extable entry this macro's asm emits
 * (EX_TYPE_UACCESS_ERR_ZERO), sets the fault-side output to 0 and
 * resumes execution at the compiled-in fixup -- i.e. this returns false
 * with *val untouched by the load, exactly the "goto efault" the C
 * source performs, driven by real hardware fault delivery, not a
 * software pointer check.
 */
__rust_helper bool rust_helper_unsafe_get_user_ul(unsigned long *val,
						   const unsigned long __user *ptr)
{
	unsigned long tmp;

	unsafe_get_user(tmp, ptr, efault);
	*val = tmp;
	return true;
efault:
	return false;
}

/*
 * lib/strncpy_from_user.c support: byte-sized sibling of
 * rust_helper_unsafe_get_user_ul above, same contract and fault
 * semantics, needed for do_strncpy_from_user()'s byte-at-a-time
 * fallback loop (unsafe_get_user(c, src+res, efault) where c is
 * `char`, not `unsigned long`).
 */
__rust_helper bool rust_helper_unsafe_get_user_u8(unsigned char *val,
						   const char __user *ptr)
{
	unsigned char tmp;

	unsafe_get_user(tmp, ptr, efault);
	*val = tmp;
	return true;
efault:
	return false;
}
