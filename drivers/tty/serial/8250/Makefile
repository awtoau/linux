# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the 8250 serial device drivers.
#

obj-$(CONFIG_SERIAL_8250)		+= 8250.o
8250-y					:= 8250_core.o
8250-y					+= 8250_platform.o
8250-$(CONFIG_SERIAL_8250_PNP)		+= 8250_pnp.o

obj-$(CONFIG_SERIAL_8250)		+= 8250_base.o
8250_base-y				:= 8250_port.o
# linux-rs: translated TU (partial) — serial8250_compute_lcr() only, see
# docs/serial-8250-translation-scoping-2026-07-18.md. 8250_port.o itself
# still builds from C either way; this just adds the Rust object it calls
# into under CONFIG_RUST.
8250_base-$(CONFIG_RUST)		+= 8250_helpers_rs.o
# linux-rs: Tier B translation — mem_serial_in()/mem_serial_out() only, see
# docs/8250-tier-b-scoping-2026-07-18.md. Compiled in and KUnit-tested
# (rust_8250_mem_serial_io suite, self-registers via .kunit_test_suites)
# but NOT called from anywhere in 8250_port.c: set_io_from_upio() still
# wires p->serial_in/p->serial_out to the C mem_serial_in/mem_serial_out,
# untouched. This object exists purely to be reachable-and-unit-testable,
# per that doc's gate, not to affect the live console path.
8250_base-$(CONFIG_RUST)		+= 8250_io_rs.o
# linux-rs: Tier C translation — serial8250_do_startup()/serial8250_do_shutdown()
# control flow only, see docs/8250-tier-c-startup-shutdown-2026-07-18.md and
# awto-au/linux-rs#25. Deliberately gated on CONFIG_RUST_8250_STARTUP, NOT
# plain CONFIG_RUST: this wires into the LIVE .startup/.shutdown uart_ops
# (unlike 8250_io_rs.o above, compiled-in but inert), has no KUnit coverage
# (real IRQ-subsystem calls, no fake possible — see that doc), and needs an
# independent on/off switch from Tier A/B's already-verified CONFIG_RUST
# wiring so the two can be A/B boot-compared without disturbing each other.
8250_base-$(CONFIG_RUST_8250_STARTUP)	+= 8250_startup_rs.o
# linux-rs: Tier C translation — serial8250_handle_irq_locked() control flow
# only, see docs/8250-tier-c-irq-2026-07-18.md and awto-au/linux-rs#25.
# Deliberately gated on CONFIG_RUST_8250_IRQ, its own independent switch
# from CONFIG_RUST and CONFIG_RUST_8250_STARTUP: this wires into the LIVE
# IRQ RX/TX dispatch path (runs on every interrupt, not just bring-up/
# teardown), has no KUnit coverage (real hardware/DMA/pm-wakeup calls, no
# fake possible — see that doc), and needs an independent on/off switch so
# each Tier C slice can be A/B boot-compared without disturbing the others.
8250_base-$(CONFIG_RUST_8250_IRQ)	+= 8250_irq_rs.o
8250_base-$(CONFIG_SERIAL_8250_DMA)	+= 8250_dma.o
8250_base-$(CONFIG_SERIAL_8250_DWLIB)	+= 8250_dwlib.o
8250_base-$(CONFIG_SERIAL_8250_FINTEK)	+= 8250_fintek.o
8250_base-$(CONFIG_SERIAL_8250_PCILIB)	+= 8250_pcilib.o
8250_base-$(CONFIG_SERIAL_8250_RSA)	+= 8250_rsa.o

obj-$(CONFIG_SERIAL_8250_CONSOLE)	+= 8250_early.o

obj-$(CONFIG_SERIAL_8250_ACCENT)	+= 8250_accent.o
obj-$(CONFIG_SERIAL_8250_ACORN)		+= 8250_acorn.o
obj-$(CONFIG_SERIAL_8250_ASPEED_VUART)	+= 8250_aspeed_vuart.o
obj-$(CONFIG_SERIAL_8250_BCM2835AUX)	+= 8250_bcm2835aux.o
obj-$(CONFIG_SERIAL_8250_BCM7271)	+= 8250_bcm7271.o
obj-$(CONFIG_SERIAL_8250_BOCA)		+= 8250_boca.o
ifeq ($(CONFIG_SERIAL_8250),y)
obj-$(CONFIG_X86_INTEL_CE)		+= 8250_ce4100.o
endif
obj-$(CONFIG_SERIAL_8250_DFL)		+= 8250_dfl.o
obj-$(CONFIG_SERIAL_8250_DW)		+= 8250_dw.o
obj-$(CONFIG_SERIAL_8250_EM)		+= 8250_em.o
obj-$(CONFIG_SERIAL_8250_EXAR)		+= 8250_exar.o
obj-$(CONFIG_SERIAL_8250_EXAR_ST16C554)	+= 8250_exar_st16c554.o
obj-$(CONFIG_SERIAL_8250_FOURPORT)	+= 8250_fourport.o
obj-$(CONFIG_SERIAL_8250_FSL)		+= 8250_fsl.o
obj-$(CONFIG_SERIAL_8250_HP300)		+= 8250_hp300.o
obj-$(CONFIG_SERIAL_8250_HUB6)		+= 8250_hub6.o
obj-$(CONFIG_SERIAL_8250_INGENIC)	+= 8250_ingenic.o
obj-$(CONFIG_SERIAL_8250_IOC3)		+= 8250_ioc3.o
obj-$(CONFIG_SERIAL_8250_KEBA)		+= 8250_keba.o
obj-$(CONFIG_SERIAL_8250_LOONGSON)	+= 8250_loongson.o
obj-$(CONFIG_SERIAL_8250_LPC18XX)	+= 8250_lpc18xx.o
obj-$(CONFIG_SERIAL_8250_LPSS)		+= 8250_lpss.o
obj-$(CONFIG_SERIAL_8250_MEN_MCB)	+= 8250_men_mcb.o
obj-$(CONFIG_SERIAL_8250_MID)		+= 8250_mid.o
obj-$(CONFIG_SERIAL_8250_MT6577)	+= 8250_mtk.o
obj-$(CONFIG_SERIAL_8250_NI)		+= 8250_ni.o
obj-$(CONFIG_SERIAL_OF_PLATFORM)	+= 8250_of.o
obj-$(CONFIG_SERIAL_8250_OMAP)		+= 8250_omap.o
obj-$(CONFIG_SERIAL_8250_PARISC)	+= 8250_parisc.o
obj-$(CONFIG_SERIAL_8250_PCI)		+= 8250_pci.o
obj-$(CONFIG_SERIAL_8250_PCI1XXXX)	+= 8250_pci1xxxx.o
obj-$(CONFIG_SERIAL_8250_PERICOM)	+= 8250_pericom.o
obj-$(CONFIG_SERIAL_8250_PXA)		+= 8250_pxa.o
obj-$(CONFIG_SERIAL_8250_RT288X)	+= 8250_rt288x.o
obj-$(CONFIG_SERIAL_8250_CS)		+= serial_cs.o
obj-$(CONFIG_SERIAL_8250_UNIPHIER)	+= 8250_uniphier.o
obj-$(CONFIG_SERIAL_8250_TEGRA)		+= 8250_tegra.o

CFLAGS_8250_ingenic.o += -I$(srctree)/scripts/dtc/libfdt
