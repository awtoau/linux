# SPDX-License-Identifier: GPL-2.0-only
obj-y += div64.o lcm.o int_log.o int_pow.o int_sqrt.o reciprocal_div.o

# linux-rs: translated TU — Rust gcd (plus C stub for the static key's
# linker-section definition) when Rust is available, C original otherwise.
ifdef CONFIG_RUST
obj-y += gcd_rs.o gcd_key.o
else
obj-y += gcd.o
endif

obj-$(CONFIG_CORDIC)		+= cordic.o
obj-$(CONFIG_POLYNOMIAL)	+= polynomial.o
obj-$(CONFIG_PRIME_NUMBERS)	+= prime_numbers.o
obj-$(CONFIG_RATIONAL)		+= rational.o

obj-$(CONFIG_TEST_DIV64)	+= test_div64.o
obj-$(CONFIG_TEST_MULDIV64)	+= test_mul_u64_u64_div_u64.o

obj-y += tests/
