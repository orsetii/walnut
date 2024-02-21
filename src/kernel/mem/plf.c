#include "mem/plf.h"

void set_taken(page_list_flags *plf)
{
  *plf |= PLF_TAKEN;
}

void set_empty(page_list_flags *plf)
{
  *plf &= ~PLF_TAKEN;
}

void set_last(page_list_flags *plf)
{
  *plf |= PLF_LAST;
}

void set_notlast(page_list_flags *plf)
{
  *plf |= PLF_LAST;
}

bool is_taken(page_list_flags *plf)
{
  return (*plf & PLF_TAKEN) == PLF_TAKEN;
}

bool is_last(page_list_flags *plf)
{
  return (*plf & (PLF_LAST)) == PLF_LAST;
}

void print_plf(page_list_flags *pfl)
{
  kprintf("TAKEN=%d LAST=%d\n", is_taken(pfl), is_last(pfl));
}