// SPDX-License-Identifier: GPL-2.0-only
/*
 * Static-key definition for gcd(), kept in C alongside the Rust
 * translation (lib/math/gcd_rs.rs): DEFINE_STATIC_KEY_TRUE places the key
 * in linker sections the Rust side cannot express yet. Written by
 * arch setup code (e.g. arch/riscv/kernel/setup.c when Zbb is absent),
 * read by gcd_rs.
 */
#include <linux/gcd.h>
#include <linux/export.h>

DEFINE_STATIC_KEY_TRUE(efficient_ffs_key);
