# SPDX-License-Identifier: GPL-2.0-only
obj-y += div64.o reciprocal_div.o

# linux-rs: translated TUs — Rust when available (gcd_key.c stays C: the
# static key's linker-section definition), C originals otherwise.
ifdef CONFIG_RUST
obj-y += gcd_rs.o gcd_key.o lcm_rs.o int_log_rs.o int_pow_rs.o int_sqrt_rs.o
else
obj-y += gcd.o lcm.o int_log.o int_pow.o int_sqrt.o
endif

obj-$(CONFIG_CORDIC)		+= cordic.o
obj-$(CONFIG_POLYNOMIAL)	+= polynomial.o
obj-$(CONFIG_PRIME_NUMBERS)	+= prime_numbers.o
ifdef CONFIG_RUST
obj-$(CONFIG_RATIONAL)		+= rational_rs.o
else
obj-$(CONFIG_RATIONAL)		+= rational.o
endif

obj-$(CONFIG_TEST_DIV64)	+= test_div64.o
obj-$(CONFIG_TEST_MULDIV64)	+= test_mul_u64_u64_div_u64.o

obj-y += tests/
