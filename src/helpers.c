#include <linux/bug.h>
#include <linux/printk.h>
#include <linux/uaccess.h>
#include <linux/version.h>
#include <linux/spinlock.h>
#include <linux/pgtable.h>

void bug_helper(void)
{
    BUG();
}

int access_ok_helper(const void __user *addr, unsigned long n)
{
#if LINUX_VERSION_CODE >= KERNEL_VERSION(5, 0, 0) /* v5.0-rc1~46 */
    return access_ok(addr, n);
#else
    return access_ok(0, addr, n);
#endif
}

void spin_lock_helper(spinlock_t *lock)
{
	spin_lock(lock);
}

void spin_unlock_helper(spinlock_t *lock)
{
	spin_unlock(lock);
}

pgd_t *pgd_offset_helper(struct mm_struct *mm, unsigned long va)
{
	return pgd_offset(mm, va);
}

int pgd_none_helper(pgd_t pgd) {
	return !!pgd_none(pgd);
}

int pgd_bad_helper(pgd_t pgd) {
	return !!pgd_bad(pgd);
}

pgdval_t pgd_val_helper(pgd_t pgd) {
	return pgd_val(pgd);
}

void pgd_set_helper(pgd_t *pgd, pgdval_t value) {
	set_pgd(pgd, __pgd(value));
}

#if LINUX_VERSION_CODE >= KERNEL_VERSION(4, 11, 0)
p4d_t *p4d_offset_helper(pgd_t *pgd, unsigned long va)
{
	return p4d_offset(pgd, va);
}

int p4d_none_helper(p4d_t p4d) {
	return !!p4d_none(p4d);
}

int p4d_bad_helper(p4d_t p4d) {
	return !!p4d_bad(p4d);
}

p4dval_t p4d_val_helper(p4d_t p4d) {
	return p4d_val(p4d);
}

void p4d_set_helper(p4d_t *p4d, p4dval_t value) {
	set_p4d(p4d, __p4d(value));
}

pud_t *pud_offset_helper(p4d_t *p4d, unsigned long va)
{
	return pud_offset(p4d, va);
}
#else
pud_t *pud_offset_helper(pgd_t *pgd, unsigned long va)
{
	return pud_offset(pgd, va);
}
#endif

int pud_none_helper(pud_t pud) {
	return !!pud_none(pud);
}

int pud_bad_helper(pud_t pud) {
	return !!pud_bad(pud);
}

pudval_t pud_val_helper(pud_t pud) {
	return pud_val(pud);
}

void pud_set_helper(pud_t *pud, pudval_t value) {
	set_pud(pud, __pud(value));
}

pmd_t *pmd_offset_helper(pud_t *pud, unsigned long va)
{
	return pmd_offset(pud, va);
}

int pmd_none_helper(pmd_t pmd) {
	return !!pmd_none(pmd);
}

int pmd_bad_helper(pmd_t pmd) {
	return !!pmd_bad(pmd);
}

pmdval_t pmd_val_helper(pmd_t pmd) {
	return pmd_val(pmd);
}

void pmd_set_helper(pmd_t *pmd, pmdval_t value) {
	set_pmd(pmd, __pmd(value));
}

pte_t *pte_offset_map_helper(pmd_t *pmd, unsigned long va)
{
	return pte_offset_map(pmd, va);
}

void pte_unmap_helper(pte_t *pte)
{
	return pte_unmap(pte);
}

int pte_none_helper(pte_t pte) {
	return !!pte_none(pte);
}

pteval_t pte_val_helper(pte_t pte) {
	return pte_val(pte);
}

void pte_set_helper(pte_t *pte, pteval_t value) {
	set_pte(pte, __pte(value));
}

/* see https://github.com/rust-lang/rust-bindgen/issues/1671 */
_Static_assert(__builtin_types_compatible_p(size_t, uintptr_t),
               "size_t must match uintptr_t, what architecture is this??");
