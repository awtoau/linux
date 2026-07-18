// SPDX-License-Identifier: GPL-2.0

#include <linux/io.h>
#include <linux/ioport.h>

__rust_helper void __iomem *rust_helper_ioremap(phys_addr_t offset, size_t size)
{
	return ioremap(offset, size);
}

__rust_helper void __iomem *rust_helper_ioremap_np(phys_addr_t offset,
						   size_t size)
{
	return ioremap_np(offset, size);
}

__rust_helper void rust_helper_iounmap(void __iomem *addr)
{
	iounmap(addr);
}

__rust_helper u8 rust_helper_readb(const void __iomem *addr)
{
	return readb(addr);
}

__rust_helper u16 rust_helper_readw(const void __iomem *addr)
{
	return readw(addr);
}

__rust_helper u32 rust_helper_readl(const void __iomem *addr)
{
	return readl(addr);
}

#ifdef CONFIG_64BIT
__rust_helper u64 rust_helper_readq(const void __iomem *addr)
{
	return readq(addr);
}
#endif

__rust_helper void rust_helper_writeb(u8 value, void __iomem *addr)
{
	writeb(value, addr);
}

__rust_helper void rust_helper_writew(u16 value, void __iomem *addr)
{
	writew(value, addr);
}

__rust_helper void rust_helper_writel(u32 value, void __iomem *addr)
{
	writel(value, addr);
}

#ifdef CONFIG_64BIT
__rust_helper void rust_helper_writeq(u64 value, void __iomem *addr)
{
	writeq(value, addr);
}
#endif

__rust_helper u8 rust_helper_readb_relaxed(const void __iomem *addr)
{
	return readb_relaxed(addr);
}

__rust_helper u16 rust_helper_readw_relaxed(const void __iomem *addr)
{
	return readw_relaxed(addr);
}

__rust_helper u32 rust_helper_readl_relaxed(const void __iomem *addr)
{
	return readl_relaxed(addr);
}

#ifdef CONFIG_64BIT
__rust_helper u64 rust_helper_readq_relaxed(const void __iomem *addr)
{
	return readq_relaxed(addr);
}
#endif

__rust_helper void rust_helper_writeb_relaxed(u8 value, void __iomem *addr)
{
	writeb_relaxed(value, addr);
}

__rust_helper void rust_helper_writew_relaxed(u16 value, void __iomem *addr)
{
	writew_relaxed(value, addr);
}

__rust_helper void rust_helper_writel_relaxed(u32 value, void __iomem *addr)
{
	writel_relaxed(value, addr);
}

#ifdef CONFIG_64BIT
__rust_helper void rust_helper_writeq_relaxed(u64 value, void __iomem *addr)
{
	writeq_relaxed(value, addr);
}
#endif

// Raw (unordered, no endian swap) MMIO accessors, distinct from the ordered
// readb/writeb family above — lib/iomem_copy.c's memset_io/memcpy_fromio/
// memcpy_toio address I/O memory word-at-a-time via these, not the ordered
// accessors, so they need their own shims rather than reusing readb/writeb.
__rust_helper u8 rust_helper_raw_readb(const volatile void __iomem *addr)
{
	return __raw_readb(addr);
}

__rust_helper u32 rust_helper_raw_readl(const volatile void __iomem *addr)
{
	return __raw_readl(addr);
}

__rust_helper void rust_helper_raw_writeb(u8 value, volatile void __iomem *addr)
{
	__raw_writeb(value, addr);
}

__rust_helper void rust_helper_raw_writel(u32 value, volatile void __iomem *addr)
{
	__raw_writel(value, addr);
}

#ifdef CONFIG_64BIT
__rust_helper u64 rust_helper_raw_readq(const volatile void __iomem *addr)
{
	return __raw_readq(addr);
}

__rust_helper void rust_helper_raw_writeq(u64 value, volatile void __iomem *addr)
{
	__raw_writeq(value, addr);
}
#endif

__rust_helper resource_size_t rust_helper_resource_size(struct resource *res)
{
	return resource_size(res);
}

__rust_helper struct resource *
rust_helper_request_mem_region(resource_size_t start, resource_size_t n,
			       const char *name)
{
	return request_mem_region(start, n, name);
}

__rust_helper void rust_helper_release_mem_region(resource_size_t start,
						  resource_size_t n)
{
	release_mem_region(start, n);
}

__rust_helper struct resource *rust_helper_request_region(resource_size_t start,
							  resource_size_t n,
							  const char *name)
{
	return request_region(start, n, name);
}

__rust_helper struct resource *
rust_helper_request_muxed_region(resource_size_t start, resource_size_t n,
				 const char *name)
{
	return request_muxed_region(start, n, name);
}

__rust_helper void rust_helper_release_region(resource_size_t start,
					      resource_size_t n)
{
	release_region(start, n);
}
