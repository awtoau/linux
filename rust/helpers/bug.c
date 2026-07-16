// SPDX-License-Identifier: GPL-2.0

#include <linux/bug.h>

__rust_helper __noreturn void rust_helper_BUG(void)
{
	BUG();
}

__rust_helper bool rust_helper_WARN_ON(bool cond)
{
	return WARN_ON(cond);
}

/* Once-per-THIS-call-site (rule 0014 shim): exact for translated TUs with
 * a single WARN_ON_ONCE site; multi-site TUs need per-site treatment. */
__rust_helper bool rust_helper_WARN_ON_ONCE(bool cond)
{
	return WARN_ON_ONCE(cond);
}
