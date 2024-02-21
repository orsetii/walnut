#include "mem/page.h"

const size_t PAGE_SIZE = 1 << 12;
const size_t PAGE_ORDER = 12;
static size_t ALLOC_START = 0;

size_t get_num_pages(void)
{
  return NUM_OF_PAGES;
}

// Align pointer to nearest 2^order bytes, rounded up
size_t align_val(size_t val, size_t order)
{
  size_t o = (1ull << order) - 1;
  return (val + o) & ~o;
}

// Get page address from page id
static size_t page_address_from_id(size_t id)
{
  return ALLOC_START + PAGE_SIZE * id;
}

void pgalloc_init()
{
  // Clear all the page flags
  zero_bytes((void *)HEAP_START, NUM_OF_PAGES);

  ALLOC_START = align_val(HEAP_START + NUM_OF_PAGES, PAGE_ORDER);
}

void *pgalloc_zalloc(uint pages)
{
  void *ptr = pgalloc_alloc(pages);

  if (ptr == NULL)
    return NULL;

  size_t sz = (PAGE_SIZE * pages) / 8;
  u64 *it = (u64 *)ptr;

  while (it++ < (u64 *)(ptr + sz))
  {
    *it = 0;
  }

  return ptr;
}

void *pgalloc_alloc(uint pages)
{
  page_list_flags *ptr = (page_list_flags *)HEAP_START;
  ASSERT(pages > 0, "Attempted to allocate 0 or less pages");

  for (size_t i = 0; i < (NUM_OF_PAGES - pages); i++)
  {
    bool found = true;

    if (!is_taken(ptr + i))
    {
      // Iterate from *i, search that we have `pages`
      // of contiguous pages
      for (size_t j = i; j < (i + pages); j++)
      {

        if (is_taken(ptr + j))
        {
          found = false;
          break;
        }
      }
    }
    else
    {
      continue;
    }

    if (found)
    {

      for (size_t pi = 0; pi <= pages - 1; pi++)
      {
        set_taken(ptr + pi);
      }

      set_last(ptr + pages - 1);

      // finally, return a pointer to the actual page itself
      return (void *)(ALLOC_START + (PAGE_SIZE * i));
    }
  }
  return NULL;
}

void pgalloc_free(void *ptr)
{
  ASSERT(ptr != NULL, "Attempted to free a NULL pointer");

  // Calculate the pagelistflags address
  page_list_flags *plf = get_plf_for_addr(ptr);

  while (is_taken(plf) && !is_last(plf))
  {
    set_empty(plf);
    plf++;
  }

  // Check and clear the last PLF for this allocation block
  ASSERT(
      is_last(plf),
      "The last in-use PLF for this allocation did not have the 'LAST' flag set");
  set_empty(plf);
  set_notlast(plf);
}

page_list_flags *get_plf_for_addr(void *ptr)
{
  return (page_list_flags *)(HEAP_START +
                             ((size_t)(ptr - ALLOC_START) / PAGE_SIZE));
}