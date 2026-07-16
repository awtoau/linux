// SPDX-License-Identifier: GPL-2.0

#include <linux/slab.h>

__rust_helper void *__must_check __realloc_size(2)
rust_helper_krealloc_node_align(const void *objp, size_t new_size, unsigned long align,
				gfp_t flags, int node)
{
	return krealloc_node_align(objp, new_size, align, flags, node);
}

__rust_helper void *__must_check __realloc_size(2)
rust_helper_kvrealloc_node_align(const void *p, size_t size, unsigned long align,
				 gfp_t flags, int node)
{
	return kvrealloc_node_align(p, size, align, flags, node);
}

/*
 * kmalloc_array() for translated TUs whose C ABI contract is "the caller
 * (e.g. argv_free) frees this with kfree" — a caller-visible detail, not
 * swappable for the kernel crate's KVVec/Box allocators. GFP_KERNEL
 * itself is already exposed as RUST_CONST_HELPER_GFP_KERNEL
 * (rust/bindings/bindings_helper.h) — reused, not re-shimmed here.
 */
__rust_helper void *__must_check __realloc_size(2)
rust_helper_kmalloc_array(size_t n, size_t size, gfp_t flags)
{
	return kmalloc_array(n, size, flags);
}
