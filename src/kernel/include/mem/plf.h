#ifndef PLF_H
#define PLF_H
#include "page.h"

#define PTE_VALID 1

void set_taken(page_list_flags *plf);
void set_empty(page_list_flags *plf);
void set_last(page_list_flags *plf);
void set_notlast(page_list_flags *plf);
bool is_taken(page_list_flags *plf);
bool is_last(page_list_flags *plf);
void print_plf(page_list_flags *pfl);

#endif