// SPDX-License-Identifier: GPL-2.0

#include <linux/llist.h>

/* linux-rs rule 0014 (tier 3): LKMM ordering primitives for the translated
 * lib/llist_rs.rs stay compiled in C, one shim per primitive per operand
 * type — never genericised. Ordering semantics are those of the C macros
 * by construction:
 *   - smp_load_acquire on the list head
 *   - READ_ONCE on a node's next pointer
 *   - try_cmpxchg on the list head (updates *old on failure, as C does)
 */
__rust_helper struct llist_node *
rust_helper_llist_load_acquire_first(struct llist_head *head)
{
	return smp_load_acquire(&head->first);
}

__rust_helper struct llist_node *
rust_helper_llist_read_once_next(struct llist_node *node)
{
	return READ_ONCE(node->next);
}

__rust_helper bool
rust_helper_llist_try_cmpxchg_first(struct llist_head *head,
				    struct llist_node **entry,
				    struct llist_node *next)
{
	return try_cmpxchg(&head->first, entry, next);
}
